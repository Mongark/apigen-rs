use serde::Deserialize;

#[derive(Deserialize)]
pub struct Route {
    pub uri: String,
    pub method: String,
    pub schema: String,
}

#[derive(Deserialize)]
pub struct SchemaType {
    pub schema_name: String,
    pub schema_type: String
}

#[derive(Deserialize)]
pub struct Schema {
    pub name: String,
    pub schema: SchemaType
}

#[derive(Deserialize)]
pub struct APIRC {
    pub name: String,
    pub schemas: Vec<String>,
    pub routes: Vec<Route>,
}
