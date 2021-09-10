set dotenv-load := true

run *args:
    cargo run {{args}}

local-cert:
    mkdir -p target
    mkcert -cert-file target/localhost.crt -key-file target/localhost.key example.com "*.example.com" localhost 127.0.0.1 ::1 $(hostname)