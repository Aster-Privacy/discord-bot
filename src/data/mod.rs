use poise::serenity_prelude as serenity;
use sqlx::{
    sqlite::SqlitePoolOptions,
    Pool,
    Sqlite,
    SqlitePool,
};

use crate::data::status_page::StatusPageSettings;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;

pub mod rss;
pub mod status_page;
pub mod utils;

#[derive(Debug, Clone)]
pub struct Data
{
    pub database: SqlitePool,
    pub client: reqwest::Client,
    pub guild: GuildSettings,
    pub status_page: StatusPageSettings,
}

#[derive(Debug, Clone)]
pub struct GuildSettings
{
    pub updates_channel: serenity::ChannelId,
    pub update_role: serenity::RoleId,
}

impl Data
{
    pub async fn new() -> Result<Self, Error>
    {
        let link = "https://status.astermail.org/".to_string();

        let database = Data::setup_db().await?;
        let client = reqwest::Client::new();

        let status_page = StatusPageSettings {
            link,
            token: std::env::var("API_TOKEN").expect("`API_TOKEN` not in env. (Better stack)"),
            page_id: std::env::var("STATUS_PAGE_ID").expect("`STATUS_PAGE_ID` not in env. (Better stack)"),
        };

        if let Ok(r) = Data::check_db(&database).await
        {
            if r
            {
                status_page.get_rss_feed(&client, &database).await?;
            };
        }

        Ok(Self {
            database,
            client,
            guild: GuildSettings {
                updates_channel: serenity::ChannelId::new(1462158478238230016),
                update_role: serenity::RoleId::new(1462158480847802420),
            },
            status_page,
        })
    }

    async fn setup_db() -> Result<Pool<Sqlite>, Error>
    {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:status.db".to_string());

        let database = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                database_url
                    .parse::<sqlx::sqlite::SqliteConnectOptions>()?
                    .create_if_missing(true),
            )
            .await?;

        Ok(database)
    }

    // Checks if the table is empty/exists
    async fn check_db(database: &Pool<Sqlite>) -> Result<bool, Error>
    {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS guids (
                id TEXT PRIMARY KEY,
                date TEXT
            )",
        )
        .execute(database)
        .await?;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM guids")
            .fetch_one(database)
            .await?;

        match count.0
        {
            0 => Ok(false),
            _ => Ok(true),
        }
    }
}
