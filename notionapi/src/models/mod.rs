use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

pub fn prop_title(content: String) -> Value {
    return json!({
        "title": [
            {
                "text": {
                    "content": content
                }
            }
        ]
    });
}

pub fn prop_rich_text(content: String) -> Value {
    return json!({
        "rich_text": [
            {
                "text": {
                    "content": content
                }
            }
        ]
    });
}

pub fn prop_select(content: String) -> Value {
    return json!({
        "select": {
            "name": content
        }
    })
}

pub fn prop_number(content: f64) -> Value {
    return json!({
        "number": content
    })
}

pub fn prop_date(date: String) -> Value {
    return json!({
        "date": {
            "start": date
        }
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parent {
    pub database_id: String,
    pub r#type: String
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub parent: Parent,
    pub properties: Map<String, Value>,
    pub children: Value,
    pub icon: Option<Value>,
    pub cover: Option<Value>,
    pub content: Option<Value>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub data: Map<String, Value>
}
