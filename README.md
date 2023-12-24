# Discord SPAM deleting Bot running on Shuttle.rs

Bot was built to delete crypto spam with "airdrop" and other related words.
Bot is able to check if characters in words where replaced by spammers with similar ones.

Change or add forbidden words in:

        let forbidden_words = ["airdrop", "ico", "token", "claim"];


In order to run initiate shuttle serenity template with:

        cargo shuttle init --template serenity

In Secrets.toml add your discord bot private key

        DISCORD_TOKEN='your key'

Start project with:

        cargo shuttle project start --idle-minutes 0

Mind you that 0 idle timer in Shuttle.rs might be changed to premium feature in future.

Deploy with:

        cargo shuttle deploy --allow-dirty
