use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};
use tokio::time::{sleep, Duration};
use regex::Regex;
use std::collections::HashMap;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let forbidden_words = ["airdrop", "ico", "giveaway"];

        // Enhanced character replacement mapping
        let mut replacement_map: HashMap<char, &str> = HashMap::new();
        replacement_map.insert('a', "[aаAА@4]");
        replacement_map.insert('i', "[i1!|IІі]");
        replacement_map.insert('r', "[rRгГ]");
        replacement_map.insert('d', "[dD]");
        replacement_map.insert('o', "[oOоО0]");
        // Add more mappings as needed

        // Construct a regex pattern to match forbidden words within larger strings
        let pattern = forbidden_words.iter().map(|&word| {
            let mut modified_word = word.to_string();
            for (english_char, replacement) in &replacement_map {
                modified_word = modified_word.chars().map(|c| {
                    if c == *english_char { replacement.to_string() } else { c.to_string() }
                }).collect();
            }
            format!(r"(?i)(\*{{1,3}}|_{{1,3}})?{}(\*{{1,3}}|_{{1,3}})?", modified_word)
        }).collect::<Vec<_>>().join("|");
        
        let regex = Regex::new(&pattern).unwrap();

        // Normalize and check the message content
        let normalized_content = unicode_normalization::UnicodeNormalization::nfc(msg.content.chars()).collect::<String>();
        if regex.is_match(&normalized_content) {
            // If a forbidden word is found in the message

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
