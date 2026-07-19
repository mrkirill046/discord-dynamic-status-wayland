use super::activity::build_activity;
use crate::config::RpcRule;
use crate::logger::Logger;
use anyhow::Result;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};

pub struct DiscordRpc {
    client: DiscordIpcClient,
}

impl DiscordRpc {
    pub fn new(client_id: &str) -> Self {
        Self {
            client: DiscordIpcClient::new(client_id),
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        self.client
            .connect()
            .map_err(|e| anyhow::anyhow!("Discord connection failed: {}", e))?;

        Ok(())
    }

    pub fn update(&mut self, rule: &RpcRule, title: &str) -> Result<()> {
        let activity = build_activity(rule, title);

        Logger::debug(&format!(
            "[RPC] state={:?}, details={:?}, large_image={:?}, large_text={:?}, small_image={:?}, small_text={:?}",
            rule.state,
            rule.details,
            rule.large_image,
            rule.large_text,
            rule.small_image,
            rule.small_text
        ));

        self.client
            .set_activity(activity)
            .map_err(|e| anyhow::anyhow!("Failed to update RPC: {}", e))?;

        Ok(())
    }
}
