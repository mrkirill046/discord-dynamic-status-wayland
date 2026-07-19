mod config;
mod constants;
mod discord;
mod hyprland;
mod logger;
mod rules;
mod system;
mod templates;

use anyhow::Result;
use discord::rpc::DiscordRpc;
use hyprland::events::listen_active_window;
use logger::Logger;

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

    listen_active_window(|class, title| {
        let rule = rules::resolve_rule(&config, &class);
        let rule = templates::apply(rule.into_owned(), &vars);

        rpc.update(&rule, &title)?;

        Ok(())
    })?;

    Ok(())
}
