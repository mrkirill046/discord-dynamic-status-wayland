mod config;
mod constants;
mod debounce;
mod discord;
mod hyprland;
mod logger;
mod rules;
mod system;
mod templates;

use anyhow::Result;
use debounce::Debouncer;
use discord::rpc::DiscordRpc;
use hyprland::events::listen_active_window;
use logger::Logger;
use std::time::Duration;

fn main() {
    if let Err(e) = app() {
        Logger::error(&format!("{:#}", e));
    }
}

fn app() -> Result<()> {
    Logger::info("Starting application...");

    let config = config::load()?;
    let system = system::get_system_info();
    let vars = templates::create_variables(&config, system);

    Logger::info("Config loaded successfully!");

    let mut rpc = DiscordRpc::new(&config.settings.app_id);

    rpc.connect()?;

    Logger::info("Connected to Discord successfully!");

    let debouncer = Debouncer::new(
        Duration::from_secs(config.settings.update_delay),
        move |class, title| {
            let rule = rules::resolve_rule(&config, &class);
            let rule = templates::apply(rule.into_owned(), &vars);

            rpc.update(&rule, &title)?;

            Ok(())
        },
    );

    listen_active_window(|class, title| {
        debouncer.send(class, title);

        Ok(())
    })?;

    Ok(())
}
