watch:
    cargo leptos watch

build:
    cargo leptos build --release

docker-build:
    sudo docker build -t music-rs:latest .

docker-run:
    sudo docker run --init --name music --rm -p 3000:3000 -v ./public/assets:/app/site/assets music-rs:latest