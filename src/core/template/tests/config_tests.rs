use super::config::{get_config_args, ConfigArg};

#[test]
fn it_should_get_config_args_from_dir() {
    let config_args = get_config_args("./folder-for-tests").unwrap();

    assert_eq!(
        config_args,
        Some(vec![
            ConfigArg {
                key: "name".to_string(),
                query: "Your name (pultzlucas): ".to_string(),
                default: Some("pultzlucas".to_string())
            },
            ConfigArg {
                key: "lastName".to_string(),
                query: "Your last name: ".to_string(),
                default: None
            }
        ]
    ))
}