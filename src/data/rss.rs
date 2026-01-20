use std::io::Cursor;

use rss::Channel;
use sqlx::SqlitePool;
use tracing::error;

use crate::data::Error;

#[derive(Debug, Clone)]
pub struct Entry
{
    pub _title: String,
    pub link: String,
    pub pub_date: Option<chrono::DateTime<chrono::Utc>>,
    pub _guid: String,
    pub description: String,
}

pub async fn check_feed(link: &str, client: &reqwest::Client, pool: &SqlitePool) -> Result<Vec<Entry>, Error>
{
    let response = client.get(format!("{}feed.rss", link)).send().await?;
    let bytes = response.bytes().await?;
    let channel = Channel::read_from(Cursor::new(bytes))?;

    let mut new_entries = Vec::new();

    for item in channel.items()
    {
        let guid = match item.guid()
        {
            Some(g) => g.value().to_string(),
            None => item.link().unwrap_or_default().to_string(),
        };

        let result = sqlx::query("INSERT OR IGNORE INTO guids (id) VALUES (?)")
            .bind(&guid)
            .execute(pool)
            .await;

        match result
        {
            Ok(query_result) =>
            {
                if query_result.rows_affected() > 0
                {
                    let date = item.pub_date().unwrap_or("");
                    let pub_date = chrono::DateTime::parse_from_rfc2822(date)
                        .ok()
                        .map(|dt| dt.with_timezone(&chrono::Utc));

                    new_entries.push(Entry {
                        _title: item.title().unwrap_or("No Title").to_string(),
                        link: item.link().unwrap_or("").to_string(),
                        pub_date,
                        _guid: guid,
                        description: item.description().unwrap_or("").to_string(),
                    });
                }
            },
            Err(e) =>
            {
                error!("Database error while checking RSS feed: {:?}", e);
            },
        }
    }

    Ok(new_entries)
}
