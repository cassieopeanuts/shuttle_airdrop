use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};
use tokio::time::{sleep, Duration};
use regex::Regex;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        // List of words to delete (in lowercase for case-insensitive matching)
        let forbidden_words = ["airdrop", "ico", "token", "claim", "airdr"];

        // Construct a regex pattern to match forbidden words in various Markdown formats
        let pattern = forbidden_words.iter().map(|&word| {
            format!(r"(?i)\b(\*{{1,3}}|_{{1,3}})?{}(\*{{1,3}}|_{{1,3}})?\b", regex::escape(word))
        }).collect::<Vec<_>>().join("|");
        let regex = Regex::new(&pattern).unwrap();

        // Check if the message contains any of the forbidden words in various formats
        if regex.is_match(&msg.content) {
            // Wait for 3 seconds before deleting the message
            sleep(Duration::from_secs(3)).await;

            // Attempt to delete the message
            if let Err(e) = msg.delete(&ctx.http).await {
                error!("Error deleting message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
