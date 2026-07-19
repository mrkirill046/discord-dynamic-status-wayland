use crate::config::{Config, RpcRule};
use std::borrow::Cow;

pub fn resolve_rule<'a>(config: &'a Config, class: &str) -> Cow<'a, RpcRule> {
    for (name, app) in &config.classes {
        let app_class = app.r#match.as_deref().unwrap_or(name);

        if app_class == class {
            return Cow::Owned(config.default.merge(&app.rule, name));
        }
    }

    Cow::Borrowed(&config.default)
}
