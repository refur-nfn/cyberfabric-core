use mini_chat_sdk::{KillSwitches, ModelCatalogEntry, TierLimits};
use modkit_macros::domain_model;

/// Service holding the model catalog loaded from configuration.
#[domain_model]
pub struct Service {
    pub catalog: Vec<ModelCatalogEntry>,
    pub kill_switches: KillSwitches,
    pub standard_limits: TierLimits,
    pub premium_limits: TierLimits,
}

impl Service {
    /// Create a service with the given configuration.
    #[must_use]
    pub fn new(
        catalog: Vec<ModelCatalogEntry>,
        kill_switches: KillSwitches,
        standard_limits: TierLimits,
        premium_limits: TierLimits,
    ) -> Self {
        Self {
            catalog,
            kill_switches,
            standard_limits,
            premium_limits,
        }
    }
}
