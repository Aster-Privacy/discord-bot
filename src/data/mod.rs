use poise::serenity_prelude as serenity;
use sqlx::{
    sqlite::SqlitePoolOptions,
    SqlitePool,
};

use crate::data::rss::check_feed;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
// pub type Context<'a> = poise::Context<'a, Data, Error>;
// pub type Command = poise::Command<Data, Error>;

pub mod rss;

pub const STATUS_URL: &str = "https://ferris.betteruptime.com/";

#[derive(Debug, Clone)]
pub struct Data
{
    pub database: SqlitePool,
    pub client: reqwest::Client,
    pub updates_channel: serenity::ChannelId,
    pub update_role: serenity::RoleId,
}

impl Data
{
    pub async fn new() -> Result<Self, Error>
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

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS guids (
                id TEXT PRIMARY KEY
            )",
        )
        .execute(&database)
        .await?;

        let client = reqwest::Client::new();

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM guids")
            .fetch_one(&database)
            .await?;

        if count.0 == 0
        {
            check_feed(&client, &database).await?;
        }

        Ok(Self {
            database,
            client,
            updates_channel: serenity::ChannelId::new(1259163562937679997),
            update_role: serenity::RoleId::new(1361849307061420055),
        })
    }
}
