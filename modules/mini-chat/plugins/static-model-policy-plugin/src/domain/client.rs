use async_trait::async_trait;
use mini_chat_sdk::{
    MiniChatModelPolicyPluginClientV1, MiniChatModelPolicyPluginError, PolicySnapshot,
    PolicyVersionInfo, PublishError, UsageEvent, UserLimits,
};
use time::OffsetDateTime;
use tracing::debug;
use uuid::Uuid;

use super::service::Service;

#[async_trait]
impl MiniChatModelPolicyPluginClientV1 for Service {
    async fn get_current_policy_version(
        &self,
        user_id: Uuid,
    ) -> Result<PolicyVersionInfo, MiniChatModelPolicyPluginError> {
        Ok(PolicyVersionInfo {
            user_id,
            policy_version: 1,
            generated_at: OffsetDateTime::now_utc(),
        })
    }

    async fn get_policy_snapshot(
        &self,
        user_id: Uuid,
        policy_version: u64,
    ) -> Result<PolicySnapshot, MiniChatModelPolicyPluginError> {
        if policy_version != 1 {
            return Err(MiniChatModelPolicyPluginError::NotFound);
        }
        Ok(PolicySnapshot {
            user_id,
            policy_version,
            model_catalog: self.catalog.clone(),
            kill_switches: self.kill_switches.clone(),
        })
    }

    async fn get_user_limits(
        &self,
        user_id: Uuid,
        policy_version: u64,
    ) -> Result<UserLimits, MiniChatModelPolicyPluginError> {
        if policy_version != 1 {
            return Err(MiniChatModelPolicyPluginError::NotFound);
        }

        Ok(UserLimits {
            user_id,
            policy_version,
            standard: self.standard_limits.clone(),
            premium: self.premium_limits.clone(),
        })
    }

    async fn publish_usage(&self, payload: UsageEvent) -> Result<(), PublishError> {
        debug!(
            turn_id = %payload.turn_id,
            tenant_id = %payload.tenant_id,
            billing_outcome = %payload.billing_outcome,
            "static plugin: publish_usage no-op"
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::StaticMiniChatPolicyPluginConfig;
    use mini_chat_sdk::ModelTier;

    fn test_service() -> Service {
        let cfg = StaticMiniChatPolicyPluginConfig::default();
        Service::new(
            cfg.model_catalog,
            cfg.kill_switches,
            cfg.standard_limits,
            cfg.premium_limits,
        )
    }

    // ── get_current_policy_version ──

    #[tokio::test]
    async fn policy_version_echoes_user_id() {
        let svc = test_service();
        let user_id = Uuid::new_v4();
        let info = svc.get_current_policy_version(user_id).await.unwrap();

        assert_eq!(info.user_id, user_id);
        assert_eq!(info.policy_version, 1);
    }

    #[tokio::test]
    async fn policy_version_timestamp_is_recent() {
        let before = OffsetDateTime::now_utc();
        let svc = test_service();
        let info = svc
            .get_current_policy_version(Uuid::new_v4())
            .await
            .unwrap();
        let after = OffsetDateTime::now_utc();

        assert!(info.generated_at >= before);
        assert!(info.generated_at <= after);
    }

    // ── get_policy_snapshot: version gating ──

    #[tokio::test]
    async fn snapshot_version_1_returns_catalog() {
        let svc = test_service();
        let user_id = Uuid::new_v4();
        let snap = svc.get_policy_snapshot(user_id, 1).await.unwrap();

        assert_eq!(snap.user_id, user_id);
        assert_eq!(snap.policy_version, 1);
        assert_eq!(snap.model_catalog.len(), 2);
    }

    #[tokio::test]
    async fn snapshot_wrong_version_returns_not_found() {
        let svc = test_service();
        for version in [0, 2, 100, u64::MAX] {
            let result = svc.get_policy_snapshot(Uuid::new_v4(), version).await;
            assert!(
                matches!(result, Err(MiniChatModelPolicyPluginError::NotFound)),
                "version {version} should return NotFound"
            );
        }
    }

    #[tokio::test]
    async fn snapshot_preserves_kill_switch_state() {
        let mut cfg = StaticMiniChatPolicyPluginConfig::default();
        cfg.kill_switches.disable_premium_tier = true;
        cfg.kill_switches.disable_web_search = true;

        let svc = Service::new(
            cfg.model_catalog,
            cfg.kill_switches,
            cfg.standard_limits,
            cfg.premium_limits,
        );
        let snap = svc.get_policy_snapshot(Uuid::new_v4(), 1).await.unwrap();

        assert!(snap.kill_switches.disable_premium_tier);
        assert!(snap.kill_switches.disable_web_search);
        assert!(!snap.kill_switches.force_standard_tier);
    }

    #[tokio::test]
    async fn snapshot_contains_both_tiers() {
        let svc = test_service();
        let snap = svc.get_policy_snapshot(Uuid::new_v4(), 1).await.unwrap();

        let has_premium = snap
            .model_catalog
            .iter()
            .any(|m| m.tier == ModelTier::Premium);
        let has_standard = snap
            .model_catalog
            .iter()
            .any(|m| m.tier == ModelTier::Standard);

        assert!(has_premium, "catalog must include a premium model");
        assert!(has_standard, "catalog must include a standard model");
    }

    // ── get_user_limits: version gating ──

    #[tokio::test]
    async fn user_limits_version_1_returns_configured_limits() {
        let svc = test_service();
        let user_id = Uuid::new_v4();
        let limits = svc.get_user_limits(user_id, 1).await.unwrap();

        assert_eq!(limits.user_id, user_id);
        assert_eq!(limits.policy_version, 1);
        // Default config: standard daily > premium daily
        assert!(
            limits.standard.limit_daily_credits_micro > limits.premium.limit_daily_credits_micro,
            "standard daily limit should exceed premium daily limit"
        );
    }

    #[tokio::test]
    async fn user_limits_wrong_version_returns_not_found() {
        let svc = test_service();
        for version in [0, 2, 100, u64::MAX] {
            let result = svc.get_user_limits(Uuid::new_v4(), version).await;
            assert!(
                matches!(result, Err(MiniChatModelPolicyPluginError::NotFound)),
                "version {version} should return NotFound"
            );
        }
    }

    #[tokio::test]
    async fn user_limits_reflect_custom_config() {
        let mut cfg = StaticMiniChatPolicyPluginConfig::default();
        cfg.standard_limits.limit_daily_credits_micro = 42;
        cfg.premium_limits.limit_monthly_credits_micro = 99;

        let svc = Service::new(
            cfg.model_catalog,
            cfg.kill_switches,
            cfg.standard_limits,
            cfg.premium_limits,
        );
        let limits = svc.get_user_limits(Uuid::new_v4(), 1).await.unwrap();

        assert_eq!(limits.standard.limit_daily_credits_micro, 42);
        assert_eq!(limits.premium.limit_monthly_credits_micro, 99);
    }
}
