# Price Fetcher in Rust project

This is a relatively simple project demonstrating how to see real-time prices such as Bitcoin are being fetched with different dependencies we can use in Rust.

Somethings this project accomplish are listed below:
- Utilizing HTTP requests in Rust.
- Utilizing JSON parsing with dependency 'serde_json'.
- Writing the data we retrieve to local files such as a txt fil.
- Implementing and utilizing traits and structures we designed in Rust.

Dependencies used are listed below:

[dependencies]
ureq = { version = "2.9", features = ["json"] }
serde = {version = "1.0", features = ["derive"]}
serde_json = "1.0"