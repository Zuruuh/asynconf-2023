fn main() {
    let schema = schemars::schema_for!(domain::Config);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
