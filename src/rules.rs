use std::borrow::Cow;
use crate::config::{Config, RpcRule};

pub fn resolve_rule<'a>(config: &'a Config, class: &str) -> Cow<'a, RpcRule> {
    match config.classes.get(class) {
        Some(custom) => Cow::Owned(config.default.merge(custom, class)),
        None => Cow::Borrowed(&config.default),
    }
}
