use std::fs::read_to_string;
use datatypes::api_types::APIRC;

pub fn get_apirc_from(address: &String) -> APIRC {
    let data: String = read_to_string(&address).unwrap();

    let apirc: APIRC = serde_json::from_str(&data).unwrap();

    apirc
}
