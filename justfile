# start docker environment
dcc-up:
    docker-compose up -d

# stop docker environment
dcc-down:
    docker-compose down

# run the server
watch-server:
    cargo watch -q -c -w src -w api -w model -x run

# run the tests
watch-test:
    cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

# format code
fmt:
    cargo fmt --all
