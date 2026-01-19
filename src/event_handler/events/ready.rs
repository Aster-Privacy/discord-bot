use std::sync::Arc;

use poise::serenity_prelude::{
    self as serenity,
    Mentionable,
};
use tracing::info;

use crate::data::{
    Data,
    Error,
    STATUS_URL,
};

pub async fn ready(http: &Arc<serenity::Http>, bot_data: &serenity::Ready, custom_data: &Arc<Data>)
    -> Result<(), Error>
{
    info!("Name: {}", bot_data.user.name);
    info!("ID: {}", bot_data.user.id.get());

    let data = custom_data.clone();
    let http = http.clone();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_mins(1));
        let components = get_component();

        loop
        {
            interval.tick().await;

            match crate::data::rss::check_feed(&data.client, &data.database).await
            {
                Ok(new_entries) =>
                {
                    if new_entries.len() == 0
                    {
                        continue;
                    }

                    for entry_number in (0..new_entries.len()).rev()
                    {
                        let entry = &new_entries[entry_number];

                        let message = format!(
                            "{}\n<t:{}:F>\n{}\n\n{}",
                            data.update_role.mention(),
                            entry
                                .pub_date
                                .map(|v| v.timestamp())
                                .unwrap_or(chrono::Utc::now().timestamp()),
                            entry.description,
                            entry.link,
                        );

                        let _ = serenity::CreateMessage::new()
                            .content(message)
                            .components(vec![components.clone()])
                            .execute(&http, data.updates_channel.widen())
                            .await;
                    }
                },
                Err(e) =>
                {
                    tracing::error!("Error checking RSS feed: {:?}", e);
                },
            }
        }
    });

    Ok(())
}

fn get_component<'a>() -> serenity::CreateComponent<'a>
{
    serenity::CreateComponent::ActionRow(serenity::CreateActionRow::Buttons(
        vec![serenity::CreateButton::new_link(STATUS_URL).label("Status Page")].into(),
    ))
}
