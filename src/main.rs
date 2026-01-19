mod data;
mod error_handler;
mod event_handler;

use std::sync::Arc;

use error_handler::on_error;
use poise::serenity_prelude as serenity;
use tracing::debug;

use crate::{
    data::Data,
    event_handler::Handler,
};

#[tokio::main]
async fn main()
{
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let options = poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            ..Default::default()
        },

        on_error: |error| Box::pin(on_error(error)),

        pre_command: |ctx| {
            Box::pin(async move {
                debug!("Executing command {}...", ctx.command().qualified_name);
            })
        },

        post_command: |ctx| {
            Box::pin(async move {
                debug!("Executed command {}!", ctx.command().qualified_name);
            })
        },

        skip_checks_for_owners: false,
        ..Default::default()
    };

    let framework = poise::Framework::new(options);

    let token = serenity::Token::from_env("DISCORD_TOKEN").expect("`DISCORD_TOKEN` not in env.");
    let intents = serenity::GatewayIntents::non_privileged();

    let data = Data::new().await.expect("Failed to initialize data");

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(Box::new(framework))
        .event_handler(Arc::new(Handler {}))
        .data(Arc::new(data))
        .await;

    client.unwrap().start().await.unwrap()
}
