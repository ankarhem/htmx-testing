run:
    npx tailwindcss -i ./src/tailwind.css -o ./public/output.css --watch &
    cargo watch -q -c -w src/ -x run | bunyan

test:
    cargo watch -x test | bunyan

build:
    npx tailwindcss -i ./src/input.css -o ./public/output.css
    cargo build --release
