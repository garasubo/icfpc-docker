use serde::Serialize;

#[derive(Debug, Serialize)]
struct Config {
    value: Option<String>,
}

fn main() {
    let config = Config {
        value: Some("hello world".to_owned()),
    };
    println!("{}", serde_json::to_string(&config).unwrap());
}
