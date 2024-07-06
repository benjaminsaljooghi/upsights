use serde_json::{Map, Value};
use up_api::v1::transactions::TransactionResource;
use notionapi::models::{Page, Parent, prop_date, prop_number, prop_rich_text, prop_select, prop_title};

pub trait PageConvertable {
    fn to_page(&self, database_id: &String) -> Page;
}

impl PageConvertable for TransactionResource {
    fn to_page(&self, database_id: &String) -> Page {
        let mut properties = Map::new();

        let title_content = self.attributes.description.clone();
        let amount_cents = self.attributes.amount.value_in_base_units;
        let amount_dollars = (amount_cents as f64) / (100.0);
        let date = self.attributes.created_at.clone();

        let category_option = &self.relationships.category.data;
        let category = match (category_option) {
            None => "uncategorised".to_string(),
            Some(category_data) => category_data.id.clone(),
        };

        properties.insert("description".to_string(), prop_title(title_content));
        properties.insert("amount".to_string(), prop_number(amount_dollars));
        properties.insert("created_at".to_string(), prop_date(date));
        properties.insert("category".to_string(), prop_select(category));

        return Page {
            parent: Parent {
                database_id: database_id.clone(),
                r#type: "database_id".parse().unwrap()
            },
            properties: properties,
            children: Value::Array(vec![]),
            icon: None,
            cover: None,
            content: None
        }
    }
}
