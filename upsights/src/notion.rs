use serde_json::{Map, Value};
use notionapi::models::{Database, Page};

pub struct Notion {
    pub config: notionapi::apis::configuration::Configuration,
    pub version: String,
}

impl Notion {

    pub fn new(token: String, version: String) -> Self {
        let mut config = notionapi::apis::configuration::Configuration::new();
        config.bearer_access_token = Some(token);
        return Self {
            config,
            version
        }
    }

    pub async fn db(&self, database_id: &String) -> Database {
        let db = notionapi::apis::databases_api::v1_databases_id_get(
            &self.config,
            &*database_id,
            Some(&*self.version)
        ).await.unwrap();

        let a: Map<String, Value> = serde_json::from_value(db).unwrap();

        return Database {
            data: a
        }
    }

    pub async fn get_page(&self, page_id: String) -> Value {
        let res = notionapi::apis::pages_api::v1_pages_id_get(
            &self.config,
            &page_id,
            Some(&self.version)
        ).await.unwrap();
        return res;
    }

    pub async fn insert_page(&self, page: Page) {

        // let body_pretty = serde_json::to_string_pretty(&page).unwrap();
        // let body = serde_json::to_string(&page).unwrap();
        // println!("sending body\n{body_pretty}");
        let res = notionapi::apis::pages_api::v1_pages_post(
            &self.config,
            None,
            None,
            Some(&*self.version),
            &page
        ).await;

        match res {
            Ok(json) => {
                println!("successful insert page: ${json:#?}")
            }
            Err(e) => {
                println!("failed insert page: ${e:#?}")
            }
        }
    }


}