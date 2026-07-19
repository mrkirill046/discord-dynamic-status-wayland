use crate::config::Config;
use crate::config::RpcRule;
use crate::system::SystemInfo;
use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

static TEMPLATE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{([^}]+)}").unwrap());

pub fn create_variables(config: &Config, system: SystemInfo) -> HashMap<String, String> {
    HashMap::from([
        ("pretty_os".into(), system.pretty_os),
        ("os".into(), system.os),
        ("wm".into(), config.settings.wm.clone()),
    ])
}

pub fn replace(value: String, vars: &HashMap<String, String>) -> String {
    TEMPLATE_REGEX
        .replace_all(&value, |caps: &regex::Captures| {
            vars.get(&caps[1])
                .cloned()
                .unwrap_or_else(|| caps[0].to_string())
        })
        .to_string()
}

pub fn apply(mut rule: RpcRule, vars: &HashMap<String, String>) -> RpcRule {
    rule.state = rule.state.map(|v| replace(v, vars));
    rule.details = rule.details.map(|v| replace(v, vars));

    rule.large_image = rule.large_image.map(|v| replace(v, vars));
    rule.large_text = rule.large_text.map(|v| replace(v, vars));

    rule.small_image = rule.small_image.map(|v| replace(v, vars));
    rule.small_text = rule.small_text.map(|v| replace(v, vars));

    rule
}
