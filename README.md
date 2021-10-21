# rustdotenv
A tool to load env files into the environment

## Install
Add to your `cargo.toml` file
```toml
[dependencies]
rustdotenv = "0.1.2"
```

## Usage
`.env` file
```text
MONGO_URI=mongodb://admin:password@127.0.0.1:27017/?authSource=admin
```
`main.rs` file
```rust
use rustdotenv::load;

fn main() {
    // If u don't provide the optional Vec<String> then it will load as default the .env file
    load(None);

    let result = std::env::var("MONGO_URI");
    if result.is_err() {
        println!("MONGO_URI env var not found");
    } else {
        println!("MONGO_URI: {}", result.unwrap())
    }
}
```