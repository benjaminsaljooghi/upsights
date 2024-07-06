use std::future::Future;
use std::str::FromStr;
use config::Config;
use serde::{Deserialize, Deserializer, Serialize};
use crate::conversions::PageConvertable;
use crate::notion::Notion;
use crate::up::Up;

mod conversions;
mod up;
mod notion;

#[derive(Deserialize)]
struct UpsightsConfig {
    up_token: String,
    notion_token: String,
    database_id: String,
    notion_version: String
}

#[tokio::main]
async fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()
        .unwrap()
        .try_deserialize::<UpsightsConfig>().unwrap();

    let notion = Notion::new(config.notion_token, config.notion_version);
    let up = Up::new(config.up_token);

    let database = notion.db(&config.database_id).await;
    let database_pretty_print = serde_json::to_string_pretty(&database).unwrap();
    println!("{database_pretty_print}");

    let page = notion.get_page("dadc54ca58f34020bc741285b37a41e9".parse().unwrap()).await;
    let page_pretty_print = serde_json::to_string_pretty(&page).unwrap();
    println!("{page_pretty_print}");

    let account_id = up.get_account_id().await;
    let spending = up.get_spending(&account_id).await;

    for spend in spending {
        notion.insert_page(spend.to_page(&config.database_id)).await;
    }
}


