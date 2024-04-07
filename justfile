version := `awk '/^\[package\]/ {p=1; next} /^\[/ {p=0} p && /version[[:space:]]*=[[:space:]]*"[^"]+"/ {gsub(/version[[:space:]]*=[[:space:]]*"/, ""); gsub(/"/, ""); print}' Cargo.toml`
engine := if `which podman > /dev/null 2>&1; echo $?` == "0" {"podman"} else {"sudo docker"}

watch:
    cargo leptos watch

build:
    cargo leptos build --release

build-container:
    {{engine}} build -t music-rs:{{version}} .

run-container:
    {{engine}} run --init --name music --rm -p 3000:3000 -v ./public/assets:/app/site/assets:ro,Z music-rs:{{version}}