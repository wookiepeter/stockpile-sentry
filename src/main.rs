use poise::serenity_prelude as serenity;

struct Data {}

type Error = Box<dyn std::error::Error + Send + Sync>;

type Context<'a> = poise::Context<'a, Data, Error>;

/// A simple command that responds with "Pong!"
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = "Pong!";
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token =
        std::env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN environment variable");

    // Define permissions required by the bot
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()], // Register commands
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Register commands globally -> potentially limit to certain guilds (i.e. guild whitelist)
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                println!("Bot is logged in as {}!", _ready.user.name);
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
