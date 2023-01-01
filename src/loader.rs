use std::fs::read_to_string;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Route {
    uri: String,
    method: String,
    schema: String,
}

#[derive(Deserialize)]
pub struct SchemaType {
    schema_name: String,
    schema_type: String
}

#[derive(Deserialize)]
pub struct Schema {
    name: String,
    schema: SchemaType
}

#[derive(Deserialize)]
pub struct APIRC {
    name: String,
    schemas: Vec<String>,
    routes: Vec<Route>,
}

pub fn get_apirc_from(address: &String) -> APIRC {
    let data: String =  read_to_string(address).unwrap();

    let apirc: APIRC = serde_json::from_str(&data).unwrap();

    apirc
}

#[cfg(test)]
mod test_loader {
    use super::get_apirc_from;

    #[test]
    fn test_get_apirc_from() {
        let address = String::from("./apirc.json");
        let apirc = get_apirc_from(&address);

        assert_eq!(apirc.name, String::from("my-api"))
    }
}
