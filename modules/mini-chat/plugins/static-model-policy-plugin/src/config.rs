use mini_chat_sdk::{KillSwitches, ModelCatalogEntry, TierLimits};
use serde::Deserialize;

/// Plugin configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct StaticMiniChatPolicyPluginConfig {
    /// Vendor name for GTS instance registration.
    pub vendor: String,

    /// Plugin priority (lower = higher priority).
    pub priority: i16,

    /// Static model catalog entries.
    pub model_catalog: Vec<ModelCatalogEntry>,

    /// Static kill switches (all disabled by default).
    pub kill_switches: KillSwitches,

    /// Per-user standard tier credit limits.
    pub standard_limits: TierLimits,

    /// Per-user premium tier credit limits.
    pub premium_limits: TierLimits,
}

impl Default for StaticMiniChatPolicyPluginConfig {
    fn default() -> Self {
        Self {
            vendor: "hyperspot".to_owned(),
            priority: 100,
            model_catalog: vec![
                default_gpt_5_2(),
                default_gpt_5_mini(),
                default_gpt_4_1(),
                default_gpt_4_1_mini(),
            ],
            kill_switches: KillSwitches::default(),
            standard_limits: TierLimits {
                limit_daily_credits_micro: 100_000_000,
                limit_monthly_credits_micro: 1_000_000_000,
            },
            premium_limits: TierLimits {
                limit_daily_credits_micro: 50_000_000,
                limit_monthly_credits_micro: 500_000_000,
            },
        }
    }
}

fn default_gpt_5_2() -> ModelCatalogEntry {
    use mini_chat_sdk::models::{
        EstimationBudgets, ModelApiParams, ModelCatalogEntry, ModelFeatures, ModelGeneralConfig,
        ModelInputType, ModelPerformance, ModelPreference, ModelSupportedEndpoints, ModelTier,
        ModelTokenPolicy, ModelToolSupport, ModelUxProfile,
    };
    use time::OffsetDateTime;
    use uuid::Uuid;

    ModelCatalogEntry {
        model_id: "gpt-5.2".to_owned(),
        display_name: "GPT-5.2".to_owned(),
        description: "Most capable model".to_owned(),
        version: "5.2".to_owned(),
        provider_id: "cti.static.openai".to_owned(),
        provider_display_name: "OpenAI".to_owned(),
        icon: String::new(),
        tier: ModelTier::Premium,
        global_enabled: true,
        multimodal_capabilities: vec!["VISION_INPUT".to_owned()],
        context_window: 128_000,
        max_output_tokens: 8_192,
        max_input_tokens: 128_000,
        input_tokens_credit_multiplier_micro: 3_000_000,
        output_tokens_credit_multiplier_micro: 15_000_000,
        multiplier_display: "3x".to_owned(),
        estimation_budgets: EstimationBudgets::default(),
        max_retrieved_chunks_per_turn: 5,
        general_config: ModelGeneralConfig {
            config_type: String::new(),
            tier: "premium".to_owned(),
            model_credential_id: Uuid::nil(),
            enabled: true,
            available_from: OffsetDateTime::UNIX_EPOCH,
            max_file_size_mb: 25,
            api_params: ModelApiParams {
                temperature: 0.7,
                top_p: 1.0,
                frequency_penalty: 0.0,
                presence_penalty: 0.0,
                stop: vec![],
            },
            features: ModelFeatures {
                streaming: true,
                function_calling: true,
                structured_output: true,
                fine_tuning: false,
                distillation: false,
                fim_completion: false,
                chat_prefix_completion: false,
            },
            input_type: ModelInputType {
                text: true,
                image: true,
                audio: false,
                video: false,
            },
            tool_support: ModelToolSupport {
                web_search: true,
                file_search: true,
                image_generation: false,
                code_interpreter: false,
                computer_use: false,
                mcp: false,
            },
            supported_endpoints: ModelSupportedEndpoints {
                chat_completions: true,
                responses: false,
                realtime: false,
                assistants: false,
                batch_api: false,
                fine_tuning: false,
                embeddings: false,
                videos: false,
                image_generation: false,
                image_edit: false,
                audio_speech_generation: false,
                audio_transcription: false,
                audio_translation: false,
                moderations: false,
                completions: false,
            },
            token_policy: ModelTokenPolicy {
                input_tokens_credit_multiplier: 3.0,
                output_tokens_credit_multiplier: 15.0,
            },
            ux_profile: ModelUxProfile {
                multiplier_display: "3x".to_owned(),
                badge_text: String::new(),
            },
            performance: ModelPerformance {
                response_latency_ms: 800,
                speed_tokens_per_second: 75,
            },
        },
        preference: ModelPreference {
            is_default: true,
            model_credential_id: Uuid::nil(),
            sort_order: 0,
        },
    }
}

fn default_gpt_5_mini() -> ModelCatalogEntry {
    use mini_chat_sdk::models::{
        EstimationBudgets, ModelApiParams, ModelCatalogEntry, ModelFeatures, ModelGeneralConfig,
        ModelInputType, ModelPerformance, ModelPreference, ModelSupportedEndpoints, ModelTier,
        ModelTokenPolicy, ModelToolSupport, ModelUxProfile,
    };
    use time::OffsetDateTime;
    use uuid::Uuid;

    ModelCatalogEntry {
        model_id: "gpt-5-mini".to_owned(),
        display_name: "GPT-5 Mini".to_owned(),
        description: "Fast and efficient model".to_owned(),
        version: "5.0".to_owned(),
        provider_id: "cti.static.openai".to_owned(),
        provider_display_name: "OpenAI".to_owned(),
        icon: String::new(),
        tier: ModelTier::Standard,
        global_enabled: true,
        multimodal_capabilities: vec![],
        context_window: 128_000,
        max_output_tokens: 4_096,
        max_input_tokens: 128_000,
        input_tokens_credit_multiplier_micro: 1_000_000,
        output_tokens_credit_multiplier_micro: 3_000_000,
        multiplier_display: "1x".to_owned(),
        estimation_budgets: EstimationBudgets::default(),
        max_retrieved_chunks_per_turn: 5,
        general_config: ModelGeneralConfig {
            config_type: String::new(),
            tier: "standard".to_owned(),
            model_credential_id: Uuid::nil(),
            enabled: true,
            available_from: OffsetDateTime::UNIX_EPOCH,
            max_file_size_mb: 25,
            api_params: ModelApiParams {
                temperature: 0.7,
                top_p: 1.0,
                frequency_penalty: 0.0,
                presence_penalty: 0.0,
                stop: vec![],
            },
            features: ModelFeatures {
                streaming: true,
                function_calling: true,
                structured_output: true,
                fine_tuning: false,
                distillation: false,
                fim_completion: false,
                chat_prefix_completion: false,
            },
            input_type: ModelInputType {
                text: true,
                image: false,
                audio: false,
                video: false,
            },
            tool_support: ModelToolSupport {
                web_search: false,
                file_search: false,
                image_generation: false,
                code_interpreter: false,
                computer_use: false,
                mcp: false,
            },
            supported_endpoints: ModelSupportedEndpoints {
                chat_completions: true,
                responses: false,
                realtime: false,
                assistants: false,
                batch_api: false,
                fine_tuning: false,
                embeddings: false,
                videos: false,
                image_generation: false,
                image_edit: false,
                audio_speech_generation: false,
                audio_transcription: false,
                audio_translation: false,
                moderations: false,
                completions: false,
            },
            token_policy: ModelTokenPolicy {
                input_tokens_credit_multiplier: 1.0,
                output_tokens_credit_multiplier: 3.0,
            },
            ux_profile: ModelUxProfile {
                multiplier_display: "1x".to_owned(),
                badge_text: String::new(),
            },
            performance: ModelPerformance {
                response_latency_ms: 400,
                speed_tokens_per_second: 150,
            },
        },
        preference: ModelPreference {
            is_default: true,
            model_credential_id: Uuid::nil(),
            sort_order: 1,
        },
    }
}

fn default_gpt_4_1() -> ModelCatalogEntry {
    use mini_chat_sdk::models::{
        EstimationBudgets, ModelApiParams, ModelCatalogEntry, ModelFeatures, ModelGeneralConfig,
        ModelInputType, ModelPerformance, ModelPreference, ModelSupportedEndpoints, ModelTier,
        ModelTokenPolicy, ModelToolSupport, ModelUxProfile,
    };
    use time::OffsetDateTime;
    use uuid::Uuid;

    ModelCatalogEntry {
        model_id: "gpt-4.1".to_owned(),
        display_name: "GPT-4.1 (Azure)".to_owned(),
        description: "GPT-4.1 on Azure OpenAI".to_owned(),
        version: "4.1".to_owned(),
        provider_id: "cti.static.azure_openai".to_owned(),
        provider_display_name: "Azure OpenAI".to_owned(),
        icon: String::new(),
        tier: ModelTier::Premium,
        global_enabled: true,
        multimodal_capabilities: vec!["VISION_INPUT".to_owned()],
        context_window: 1_047_576,
        max_output_tokens: 32_768,
        max_input_tokens: 1_047_576,
        input_tokens_credit_multiplier_micro: 2_000_000,
        output_tokens_credit_multiplier_micro: 8_000_000,
        multiplier_display: "2x".to_owned(),
        estimation_budgets: EstimationBudgets::default(),
        max_retrieved_chunks_per_turn: 5,
        general_config: ModelGeneralConfig {
            config_type: String::new(),
            tier: "premium".to_owned(),
            model_credential_id: Uuid::nil(),
            enabled: true,
            available_from: OffsetDateTime::UNIX_EPOCH,
            max_file_size_mb: 25,
            api_params: ModelApiParams {
                temperature: 0.7,
                top_p: 1.0,
                frequency_penalty: 0.0,
                presence_penalty: 0.0,
                stop: vec![],
            },
            features: ModelFeatures {
                streaming: true,
                function_calling: true,
                structured_output: true,
                fine_tuning: false,
                distillation: false,
                fim_completion: false,
                chat_prefix_completion: false,
            },
            input_type: ModelInputType {
                text: true,
                image: true,
                audio: false,
                video: false,
            },
            tool_support: ModelToolSupport {
                web_search: true,
                file_search: true,
                image_generation: false,
                code_interpreter: false,
                computer_use: false,
                mcp: false,
            },
            supported_endpoints: ModelSupportedEndpoints {
                chat_completions: true,
                responses: false,
                realtime: false,
                assistants: false,
                batch_api: false,
                fine_tuning: false,
                embeddings: false,
                videos: false,
                image_generation: false,
                image_edit: false,
                audio_speech_generation: false,
                audio_transcription: false,
                audio_translation: false,
                moderations: false,
                completions: false,
            },
            token_policy: ModelTokenPolicy {
                input_tokens_credit_multiplier: 2.0,
                output_tokens_credit_multiplier: 8.0,
            },
            ux_profile: ModelUxProfile {
                multiplier_display: "2x".to_owned(),
                badge_text: String::new(),
            },
            performance: ModelPerformance {
                response_latency_ms: 600,
                speed_tokens_per_second: 100,
            },
        },
        preference: ModelPreference {
            is_default: false,
            model_credential_id: Uuid::nil(),
            sort_order: 2,
        },
    }
}

fn default_gpt_4_1_mini() -> ModelCatalogEntry {
    use mini_chat_sdk::models::{
        EstimationBudgets, ModelApiParams, ModelCatalogEntry, ModelFeatures, ModelGeneralConfig,
        ModelInputType, ModelPerformance, ModelPreference, ModelSupportedEndpoints, ModelTier,
        ModelTokenPolicy, ModelToolSupport, ModelUxProfile,
    };
    use time::OffsetDateTime;
    use uuid::Uuid;

    ModelCatalogEntry {
        model_id: "gpt-4.1-mini".to_owned(),
        display_name: "GPT-4.1 Mini (Azure)".to_owned(),
        description: "GPT-4.1 Mini on Azure OpenAI".to_owned(),
        version: "4.1".to_owned(),
        provider_id: "cti.static.azure_openai".to_owned(),
        provider_display_name: "Azure OpenAI".to_owned(),
        icon: String::new(),
        tier: ModelTier::Standard,
        global_enabled: true,
        multimodal_capabilities: vec![],
        context_window: 1_047_576,
        max_output_tokens: 32_768,
        max_input_tokens: 1_047_576,
        input_tokens_credit_multiplier_micro: 400_000,
        output_tokens_credit_multiplier_micro: 1_600_000,
        multiplier_display: "0.4x".to_owned(),
        estimation_budgets: EstimationBudgets::default(),
        max_retrieved_chunks_per_turn: 5,
        general_config: ModelGeneralConfig {
            config_type: String::new(),
            tier: "standard".to_owned(),
            model_credential_id: Uuid::nil(),
            enabled: true,
            available_from: OffsetDateTime::UNIX_EPOCH,
            max_file_size_mb: 25,
            api_params: ModelApiParams {
                temperature: 0.7,
                top_p: 1.0,
                frequency_penalty: 0.0,
                presence_penalty: 0.0,
                stop: vec![],
            },
            features: ModelFeatures {
                streaming: true,
                function_calling: true,
                structured_output: true,
                fine_tuning: false,
                distillation: false,
                fim_completion: false,
                chat_prefix_completion: false,
            },
            input_type: ModelInputType {
                text: true,
                image: false,
                audio: false,
                video: false,
            },
            tool_support: ModelToolSupport {
                web_search: false,
                file_search: false,
                image_generation: false,
                code_interpreter: false,
                computer_use: false,
                mcp: false,
            },
            supported_endpoints: ModelSupportedEndpoints {
                chat_completions: true,
                responses: false,
                realtime: false,
                assistants: false,
                batch_api: false,
                fine_tuning: false,
                embeddings: false,
                videos: false,
                image_generation: false,
                image_edit: false,
                audio_speech_generation: false,
                audio_transcription: false,
                audio_translation: false,
                moderations: false,
                completions: false,
            },
            token_policy: ModelTokenPolicy {
                input_tokens_credit_multiplier: 0.4,
                output_tokens_credit_multiplier: 1.6,
            },
            ux_profile: ModelUxProfile {
                multiplier_display: "0.4x".to_owned(),
                badge_text: String::new(),
            },
            performance: ModelPerformance {
                response_latency_ms: 300,
                speed_tokens_per_second: 200,
            },
        },
        preference: ModelPreference {
            is_default: false,
            model_credential_id: Uuid::nil(),
            sort_order: 3,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mini_chat_sdk::ModelTier;

    // ── Default catalog invariants ──
    // The default config is the development/testing fallback.
    // These invariants ensure it provides a usable baseline.

    #[test]
    fn default_catalog_contains_two_models_per_tier() {
        let cfg = StaticMiniChatPolicyPluginConfig::default();
        let premium_count = cfg
            .model_catalog
            .iter()
            .filter(|m| m.tier == ModelTier::Premium)
            .count();
        let standard_count = cfg
            .model_catalog
            .iter()
            .filter(|m| m.tier == ModelTier::Standard)
            .count();

        assert_eq!(
            premium_count, 2,
            "default catalog must have exactly 2 premium models"
        );
        assert_eq!(
            standard_count, 2,
            "default catalog must have exactly 2 standard models"
        );
    }

    #[test]
    fn default_catalog_all_models_globally_enabled() {
        let cfg = StaticMiniChatPolicyPluginConfig::default();
        for model in &cfg.model_catalog {
            assert!(
                model.global_enabled,
                "default model '{}' must be globally enabled",
                model.model_id
            );
        }
    }

    #[test]
    fn default_premium_model_has_higher_credit_multiplier_than_standard() {
        let cfg = StaticMiniChatPolicyPluginConfig::default();
        let premium = cfg
            .model_catalog
            .iter()
            .find(|m| m.tier == ModelTier::Premium)
            .unwrap();
        let standard = cfg
            .model_catalog
            .iter()
            .find(|m| m.tier == ModelTier::Standard)
            .unwrap();

        assert!(
            premium.output_tokens_credit_multiplier_micro
                > standard.output_tokens_credit_multiplier_micro,
            "premium output multiplier ({}) must exceed standard ({})",
            premium.output_tokens_credit_multiplier_micro,
            standard.output_tokens_credit_multiplier_micro,
        );
    }

    // ── Credit limit domain invariants ──

    #[test]
    fn default_standard_limits_exceed_premium_limits() {
        let cfg = StaticMiniChatPolicyPluginConfig::default();
        assert!(
            cfg.standard_limits.limit_daily_credits_micro
                > cfg.premium_limits.limit_daily_credits_micro,
            "standard daily credits must exceed premium daily credits"
        );
        assert!(
            cfg.standard_limits.limit_monthly_credits_micro
                > cfg.premium_limits.limit_monthly_credits_micro,
            "standard monthly credits must exceed premium monthly credits"
        );
    }

    #[test]
    fn default_monthly_limits_exceed_daily_limits() {
        let cfg = StaticMiniChatPolicyPluginConfig::default();
        assert!(
            cfg.standard_limits.limit_monthly_credits_micro
                > cfg.standard_limits.limit_daily_credits_micro,
            "standard monthly must exceed standard daily"
        );
        assert!(
            cfg.premium_limits.limit_monthly_credits_micro
                > cfg.premium_limits.limit_daily_credits_micro,
            "premium monthly must exceed premium daily"
        );
    }

    // ── deny_unknown_fields ──

    #[test]
    fn config_deserialize_rejects_unknown_fields() {
        let json = r#"{"vendor": "test", "unknown_field": true}"#;
        let result: Result<StaticMiniChatPolicyPluginConfig, _> = serde_json::from_str(json);
        assert!(
            result.is_err(),
            "deny_unknown_fields should reject unknown keys"
        );
    }

    #[test]
    fn config_deserialize_empty_uses_defaults() {
        let cfg: StaticMiniChatPolicyPluginConfig = serde_json::from_str("{}").unwrap();
        assert_eq!(cfg.vendor, "hyperspot");
        assert_eq!(cfg.priority, 100);
        assert_eq!(cfg.model_catalog.len(), 4);
    }

    #[test]
    fn default_kill_switches_all_off() {
        let cfg = StaticMiniChatPolicyPluginConfig::default();
        assert!(!cfg.kill_switches.disable_premium_tier);
        assert!(!cfg.kill_switches.force_standard_tier);
        assert!(!cfg.kill_switches.disable_web_search);
        assert!(!cfg.kill_switches.disable_file_search);
        assert!(!cfg.kill_switches.disable_images);
    }
}
