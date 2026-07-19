use crate::config::RpcRule;
use discord_rich_presence::activity;

pub fn build_activity<'a>(rule: &'a RpcRule, title: &'a str) -> activity::Activity<'a> {
    let mut act = activity::Activity::new();

    if let Some(state) = &rule.state {
        act = act.state(state);
    }

    if rule.details_from_title.unwrap_or(false) {
        act = act.details(title);
    } else if let Some(details) = &rule.details {
        act = act.details(details);
    }

    if rule.large_image.is_some() || rule.small_image.is_some() {
        let mut assets = activity::Assets::new();

        if let Some(v) = &rule.large_image {
            assets = assets.large_image(v);
        }

        if let Some(v) = &rule.large_text {
            assets = assets.large_text(v);
        }

        if let Some(v) = &rule.small_image {
            assets = assets.small_image(v);
        }

        if let Some(v) = &rule.small_text {
            assets = assets.small_text(v);
        }

        act = act.assets(assets);
    }

    act
}
