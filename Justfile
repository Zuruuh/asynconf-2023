schema:
    cargo run --bin schema -q > schema.json

test *args:
  cargo test {{args}} -- --nocapture
