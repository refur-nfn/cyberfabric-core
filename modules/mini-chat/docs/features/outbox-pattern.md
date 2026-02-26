 # Feature: Transactional Outbox Pattern
- [ ] `p1` - **ID**: `cpt-cf-mini-chat-featstatus-usage-outbox`

<!-- reference to DECOMPOSITION entry -->
- [ ] `p1` - `cpt-cf-mini-chat-feature-usage-outbox`

## 1. Feature Context

### 1.1 Overview

This feature describes the transactional outbox pattern implemented in `modkit-db` (shared infra table `modkit_outbox_events`) so that event publishing is reliable and does not add synchronous network calls to the critical execution hot path.

The outbox is a **general-purpose infrastructure mechanism**.
Modules use it by publishing messages under a dedicated `(namespace, topic)` pair.

### 1.2 Purpose

This feature ensures the **Outbox Completeness Invariant**: it MUST be impossible for committed side effects to exist without a corresponding persisted outbox event.

It also ensures events can be delivered asynchronously with at-least-once delivery semantics.

### 1.3 Actors

| Actor | Role in Feature |
|-------|-----------------|
| `cpt-cf-mini-chat-actor-chat-user` | Initiates an operation whose commit MUST enqueue an outbox event (when side effects are applied). |
| `cpt-cf-mini-chat-actor-usage-outbox-dispatcher` | Background worker that claims pending outbox rows and publishes events to a downstream consumer with retries. |
| `cpt-cf-mini-chat-actor-outbox-consumer` | Downstream event consumer that processes deliveries idempotently. |

### 1.4 References

- ModKit lifecycle/stateful tasks documentation (stateful worker)

### 1.5 Implementation Shape (normative)

- Outbox events are enqueued through `modkit_db::outbox::enqueue(runner, msg)` where `runner: &impl DBRunner`.
- The enqueue call MUST run inside the same DB transaction as the side effects the event describes.
- A background dispatcher (ModKit `stateful` lifecycle task) claims events from `modkit_outbox_events` using a lease (`locked_by`, `locked_until`) and delivers them with retries.
- Producers SHOULD provide a stable `dedupe_key` to support idempotent enqueue and idempotent downstream processing.

### 1.6 Outbox Storage (normative)

Outbox events are stored in a shared infrastructure table owned by `modkit-db`:

`modkit_outbox_events`

**Columns (minimum)**:

- `id uuid pk`
- `namespace text not null`
- `topic text not null`
- `tenant_id uuid null`
- `dedupe_key text null`
- `payload jsonb not null`
- `status text not null` (`pending|processing|delivered|dead`)
- `attempts int not null default 0`
- `next_attempt_at timestamptz not null default now()`
- `locked_by uuid null`
- `locked_until timestamptz null`
- `last_error text null`
- `created_at timestamptz not null default now()`
- `updated_at timestamptz not null default now()`

**Indexes (minimum)**:

- `index(status, next_attempt_at)`
- `index(locked_until)`

**Dedupe / idempotency (Postgres)**:

- Partial unique index on `(namespace, topic, dedupe_key)` where `dedupe_key IS NOT NULL`.

### 1.7 Proposed `modkit_db::outbox` v1 API (sketch)

This is the intended interface shape for the generalized outbox mechanism in `modkit-db`.
Modules consume this API with their own `namespace/topic`.

```rust
use modkit_db::secure::DBRunner;
use modkit_db::DBProvider;
use serde_json::Value;
use std::time::Duration;
use uuid::Uuid;

pub struct OutboxMessage {
    pub namespace: &'static str,
    pub topic: &'static str,
    pub tenant_id: Option<Uuid>,
    pub dedupe_key: Option<String>,
    pub payload: Value,
}

pub async fn enqueue(
    runner: &impl DBRunner,
    msg: OutboxMessage,
) -> Result<Uuid, modkit_db::DbError>;

pub struct ClaimCfg {
    pub batch_size: u32,
    pub lease_duration: Duration,
    pub max_attempts: u32,
}

pub struct ClaimedMessage {
    pub id: Uuid,
    pub namespace: String,
    pub topic: String,
    pub tenant_id: Option<Uuid>,
    pub dedupe_key: Option<String>,
    pub payload: Value,
    pub attempts: i32,
}

pub struct OutboxStore<E> {
    pub db: DBProvider<E>,
    pub worker_id: Uuid,
    pub namespace: String,
}

impl<E> OutboxStore<E>
where
    E: From<modkit_db::DbError> + Send + 'static,
{
    pub async fn claim_batch(&self, cfg: ClaimCfg) -> Result<Vec<ClaimedMessage>, E>;
    pub async fn ack(&self, id: Uuid) -> Result<(), E>;
    pub async fn nack(&self, id: Uuid, err: &str) -> Result<(), E>;
}
```

## 2. Actor Flows (CDSL)

### Operation Commit Enqueues Outbox Row

- [ ] `p1` - **ID**: `cpt-cf-mini-chat-flow-usage-outbox-enqueue`

**Actor**: `cpt-cf-mini-chat-actor-chat-user`

**Success Scenarios**:
 - An operation commits, and exactly one `modkit_outbox_events` row is inserted atomically.

**Error Scenarios**:
- The DB transaction fails: the described side effects and outbox insertion MUST both roll back.

**Behavior (normative)**:
- The outbox row insertion is part of the operation's commit: it MUST be in the **same DB transaction** as the committed side effects.
- Within that transaction:
  - The system MUST enqueue exactly one outbox event describing the committed side effects.
- If an operation implementation uses a uniqueness guard for idempotent enqueue (e.g. a stable `dedupe_key` with a unique index):
  - A conflict on insert MUST be treated as "already enqueued".
- If the transaction fails/rolls back for any reason:
  - No `modkit_outbox_events` row is persisted.

**Payload requirements**:
- The outbox payload MUST include sufficient identifiers for idempotent downstream processing.
- The outbox row SHOULD include a stable `dedupe_key` suitable for idempotency.

### Outbox Dispatcher Publishes Events

- [ ] `p1` - **ID**: `cpt-cf-mini-chat-flow-usage-outbox-dispatch`

**Actor**: `cpt-cf-mini-chat-actor-usage-outbox-dispatcher`

**Success Scenarios**:
 - Pending outbox rows are claimed using `FOR UPDATE SKIP LOCKED` and published to the downstream consumer.
- Claimed rows are marked `delivered` on success.

**Error Scenarios**:
- Publish fails (temporary): the row is returned to `pending` and scheduled for retry using backoff.
- Worker crashes after claiming: rows are reclaimed after lease expiry.

**Behavior (normative)**:
- The dispatcher is an internal background worker (implemented as a ModKit stateful lifecycle task).
- It MUST periodically poll for publishable outbox rows and process them until a shutdown `CancellationToken` is triggered.
- Claiming MUST be safe under concurrency (multiple replicas/workers) and MUST use row-level locking via `FOR UPDATE SKIP LOCKED`.
- Claimed rows MUST be leased using `(locked_by, locked_until)` so that:
  - A crashing worker does not permanently strand a row.
  - Another worker can reclaim a row after lease expiry.
- For each claimed row, the dispatcher MUST publish the outbox payload to the downstream consumer.
- On publish success, the row MUST transition to `delivered` and be made ineligible for further dispatch.
- On publish failure, the row MUST be returned to `pending` and rescheduled by setting `next_attempt_at` using a retry policy, while recording `attempts` and `last_error`.

**Implementation note (normative)**:

- The dispatcher uses a `modkit_db::outbox::OutboxStore<E>` constructed from a `DBProvider<E>`.
- The store provides:
  - `claim_batch(...) -> Vec<ClaimedMessage>`
  - `ack(id)`
  - `nack(id, err)`
- `ack` MUST only succeed for rows currently leased by the same worker (guarded by `locked_by`).

**Idempotency requirement**:
- The dispatcher MUST assume at-least-once delivery.
- Downstream processing MUST be idempotent on a stable key (e.g. the outbox `dedupe_key`).

## 3. Processes / Business Logic (CDSL)

### Enqueue Outbox Row (Transactional)

- [ ] `p1` - **ID**: `cpt-cf-mini-chat-algo-usage-outbox-enqueue`

**Input**:
- Operation outcome (completed/failed/aborted)
- Committed side effects summary (module-defined)
- Identifiers used for idempotency and downstream correlation

**Output**:
- Persisted `modkit_outbox_events` row inserted atomically with the committed side effects

**Requirements**:
- The enqueue operation MUST run inside the same DB transaction as the described side effects.
- The outbox payload MUST be derived from already-validated internal state (no client-provided usage fields).
- Enqueue MUST be idempotent on `dedupe_key` when it is provided:
  - Multiple attempts to enqueue the same logical event MUST NOT produce multiple outbox rows.
  - This is implemented in storage via the dedupe unique index and an upsert/ignore-on-conflict insert.
- The outbox payload MUST include all information needed by the downstream consumer.
- The outbox row MUST be initialized with:
  - `namespace` and `topic` appropriate for the producer
  - `status = 'pending'`
  - `attempts = 0`
  - `next_attempt_at = now()`
  - `locked_by = NULL`, `locked_until = NULL`

### Claim Pending Outbox Rows (Lease + Skip Locked)

- [ ] `p1` - **ID**: `cpt-cf-mini-chat-algo-usage-outbox-claim`

**Input**:
- batch_size
- lease_duration
- worker_id

**Output**:
- A list of claimed rows ready to publish

**Requirements**:
- Claim MUST be performed in a DB transaction.
- Only rows eligible for dispatch MAY be claimed:
  - `status = 'pending'`
  - `next_attempt_at <= now()`
  - and either the row is unclaimed, or any previous lease is expired.
- The claim query MUST lock selected rows using `FOR UPDATE SKIP LOCKED`.
- Upon claim, the worker MUST:
  - transition row to `processing`
  - increment `attempts`
  - set `locked_by = worker_id`
  - set `locked_until = now() + lease_duration`
- Claim ordering MUST be stable and low-risk for starvation (e.g., order by `created_at`).
- Claim MUST support multiple workers without double-claiming the same row.

### Retry Scheduling on Publish Failure

- [ ] `p1` - **ID**: `cpt-cf-mini-chat-algo-usage-outbox-retry`

**Input**:
- publish error
- current attempts count
- retry policy configuration (max_attempts, base_delay, max_delay)

**Output**:
- Updated outbox row with next attempt time

**Requirements**:
- On publish failure, the dispatcher MUST record `last_error`.
- The dispatcher MUST clear any claim lease when rescheduling: `locked_by = NULL`, `locked_until = NULL`.
- The dispatcher MUST compute `next_attempt_at` using exponential backoff with jitter, bounded by a configured maximum.
- If `attempts` exceeds `max_attempts`, the dispatcher MUST transition the row to `dead` and MUST NOT retry it automatically.

## 4. States (CDSL)

### `modkit_outbox_events` Row State Machine

- [ ] `p1` - **ID**: `cpt-cf-mini-chat-state-usage-outbox-row`

**States**: pending, processing, delivered, dead

**Initial State**: pending

**State semantics (normative)**:
- `pending`:
  - Row is eligible for claiming when `next_attempt_at <= now()`.
  - Row MUST NOT be published unless it is first claimed.
- `processing`:
  - Row is claimed by a dispatcher and has an active lease (`locked_until`).
  - Row may be re-published in crash scenarios (at-least-once delivery).
  - If `now() > locked_until`, the row becomes reclaimable (it may be transitioned back to `pending`).
- `delivered`:
  - Terminal state.
  - Row MUST NOT transition out of `delivered`.
- `dead`:
  - Terminal state for permanent failures (attempts exceeded `max_attempts`).
  - Row MUST NOT be retried automatically.

## 5. Definitions of Done

### Provide Transactional Outbox Persistence

- [ ] `p1` - **ID**: `cpt-cf-mini-chat-dod-usage-outbox-transactional`

The system **MUST** persist a `modkit_outbox_events` row in the same DB transaction as any committed side effects described by that event.

**Implements**:
- `cpt-cf-mini-chat-flow-usage-outbox-enqueue`
- `cpt-cf-mini-chat-algo-usage-outbox-enqueue`

**Touches**:
- DB: `modkit_outbox_events`

### Provide Stateful Usage Outbox Dispatcher

- [ ] `p1` - **ID**: `cpt-cf-mini-chat-dod-usage-outbox-dispatcher`

The system **MUST** run a background dispatcher as a stateful lifecycle task that:
- Claims rows using `FOR UPDATE SKIP LOCKED`.
- Uses a lease (`locked_until`) to ensure rows are recoverable after crashes.
- Retries failed publishes using backoff and records `last_error`.

**Implements**:
- `cpt-cf-mini-chat-flow-usage-outbox-dispatch`
- `cpt-cf-mini-chat-algo-usage-outbox-claim`
- `cpt-cf-mini-chat-algo-usage-outbox-retry`
- `cpt-cf-mini-chat-state-usage-outbox-row`

**Touches**:
- DB: `modkit_outbox_events`

### Enforce Idempotent Publish Contract

- [ ] `p1` - **ID**: `cpt-cf-mini-chat-dod-usage-outbox-idempotency`

The system **MUST** ensure that event delivery is safe under retries and replays by using a stable dedupe key and requiring downstream processing to be idempotent on that key.

**Implements**:
- `cpt-cf-mini-chat-flow-usage-outbox-dispatch`

**Touches**:
- DB: `modkit_outbox_events` (dedupe key)

## 6. Acceptance Criteria

- [ ] For any committed side effects that require an event, there exists exactly one corresponding persisted `modkit_outbox_events` row (same DB transaction boundary).
- [ ] If a producer loses an idempotency race, it does not insert a duplicate `modkit_outbox_events` row.
- [ ] Dispatcher can run concurrently (multiple replicas) without double-processing rows (verified via `SKIP LOCKED` + lease).
- [ ] If the dispatcher crashes after claiming rows, those rows become eligible for reclaim after lease expiry.
- [ ] Publish failures reschedule rows with increasing `next_attempt_at` and record `last_error`.
- [ ] Publishing is safe under retries (downstream processing is idempotent on dedupe key).
