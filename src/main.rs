extern crate dotenv;

use dotenv::dotenv;
pub mod commands;

#[tokio::main]
async fn main() {
    dotenv().ok();
    poise::Framework::build()
        .token(std::env::var("DISCORD_BOT_TOKEN").unwrap())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(()) }))
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::age::age(),
                commands::collection::collection(),
                commands::register::register(),
            ],
            // configure framework here
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        
        .run().await.unwrap();
}