mod config;
mod constants;
mod debounce;
mod discord;
mod logger;
mod rules;
mod system;
mod templates;
mod wm;

use anyhow::Result;
use debounce::Debouncer;
use discord::rpc::DiscordRpc;
use logger::Logger;
use std::time::Duration;
use wm::listen_active_window;

fn main() {
    if let Err(e) = app() {
        Logger::error(&format!("{:#}", e));
    }
}

fn app() -> Result<()> {
    Logger::info("Starting application...");

    let config = config::load()?;
    let settings = config.settings.clone();
    let system = system::get_system_info();
    let vars = templates::create_variables(&config, system);

    Logger::info("Config loaded successfully!");

    let mut rpc = DiscordRpc::new(&config.settings.app_id);

    loop {
        match rpc.connect() {
            Ok(_) => break,
            Err(e) => {
                Logger::warn(&format!(
                    "Discord is not running yet, or there may be an internet connectivity issue: {e}"
                ));

                std::thread::sleep(Duration::from_secs(2));
            }
        }
    }

    Logger::info("Connected to Discord successfully!");

    let debouncer = Debouncer::new(
        Duration::from_secs(settings.update_delay),
        move |class, title| {
            let rule = rules::resolve_rule(&config, &class);
            let rule = templates::apply(rule.into_owned(), &vars);

            rpc.update(&rule, &title)?;

            Ok(())
        },
    );

    listen_active_window(&settings, |class, title| {
        debouncer.send(class, title);

        Ok(())
    })?;

    Ok(())
}
