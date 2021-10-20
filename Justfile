set dotenv-load := true

# Run the application
run *args:
    cargo run {{args}}

# Reset the database, then run the application
run-reset *args:
    just reset
    cargo run {{args}}

# Remove all data from the database, then run migrations
reset:
    sqlx db reset -y

# Watch for something
# E.g. `just watch run --release` or `just watch test`
watch *args:
    cargo watch --shell 'just {{ args }}'

build *args:
    cargo build --release 

# Create a self-signed certificate for testing
# See .env.example for configuring the app to use it
local-cert:
    mkdir -p target
    mkcert -cert-file target/localhost.crt -key-file target/localhost.key example.com "*.example.com" localhost 127.0.0.1 ::1 $(hostname)

test *args:
    cargo test -- -q {{args}}

lint:
    cargo clippy --release -- -D warnings
    cargo clippy -- -D warnings