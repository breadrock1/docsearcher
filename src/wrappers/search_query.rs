use serde_derive::Serialize;

#[derive(Serialize)]
struct QueryString {
    query: String,
    operator: String,
    fields: Vec<String>,
}

impl QueryString {
    pub fn new(value: &str) -> Self {
        QueryString {
            query: value.to_string(),
            operator: "or".to_string(),
            fields: vec![
                "entity_data".to_string(),
                "document_path".to_string(),
            ]
        }
    }
}

#[derive(Serialize)]
pub struct MultiMatchQuery {
    multi_match: QueryString,
}

impl MultiMatchQuery {
    pub fn new(value: &str) -> Self {
        MultiMatchQuery {
            multi_match: QueryString::new(value),
        }
    }
}
