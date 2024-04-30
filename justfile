# A shortcut for now to run these in parallel
dev-all:
    just dev-api & just dev-web

dev-api:
    cargo run --bin api

dev-web:
    yarn workspace web run dev