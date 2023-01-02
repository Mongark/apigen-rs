use std::fs::read_to_string;
use datatypes::api_types::APIRC;

pub fn get_apirc_from(address: &String) -> APIRC {
    let data: String = read_to_string(&address)
        .expect("Error reading file");
    let apirc: APIRC = serde_json::from_str(&data)
        .expect("Invalid config file");

    apirc
}
