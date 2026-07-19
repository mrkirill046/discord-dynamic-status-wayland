mod loader;
mod path;

use serde::Deserialize;
use std::collections::HashMap;

pub use loader::load;

#[derive(Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub default: RpcRule,
    pub classes: HashMap<String, AppRule>,
}

#[derive(Deserialize, Clone)]
pub struct RpcRule {
    pub state: Option<String>,
    pub details: Option<String>,

    pub details_from_title: Option<bool>,

    pub large_image: Option<String>,
    pub large_text: Option<String>,

    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct AppRule {
    pub r#match: Option<String>,

    #[serde(flatten)]
    pub rule: RpcRule,
}

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub app_id: String,
    pub wm: String,
    pub update_delay: u64,
}

impl RpcRule {
    pub fn merge(&self, override_rule: &RpcRule, app_name: &str) -> RpcRule {
        RpcRule {
            state: override_rule.state.clone().or(self.state.clone()),

            details: override_rule.details.clone().or(self.details.clone()),
            details_from_title: override_rule.details_from_title.or(self.details_from_title),

            large_image: override_rule
                .large_image
                .clone()
                .or(self.large_image.clone()),

            large_text: override_rule.large_text.clone().or(self.large_text.clone()),

            small_image: override_rule
                .small_image
                .clone()
                .or(Some(app_name.to_string())),

            small_text: override_rule.small_text.clone().or(self.small_text.clone()),
        }
    }
}
