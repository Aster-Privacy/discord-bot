use crate::data::{
    Command,
    Context,
    Error,
};

mod status;

/// Registers your commands
#[poise::command(
    prefix_command,
    slash_command,
    owners_only,
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn register_commands(ctx: Context<'_>) -> Result<(), Error>
{
    let commands = &ctx.framework().options().commands;
    poise::builtins::register_globally(ctx.http(), commands).await?;

    ctx.say("Successfully registered slash commands!").await?;
    Ok(())
}

pub fn get_commands() -> Vec<Command>
{
    vec![register_commands(), status::status()]
}
