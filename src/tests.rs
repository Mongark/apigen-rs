#[cfg(test)]
mod test_loader {
    use config::loader::get_apirc_from;

    #[test]
    fn test_get_apirc_from() {
        let address = String::from("./apirc.json");
        let apirc = get_apirc_from(&address);

        assert_eq!(apirc.name, String::from("my-api"))
    }
}
