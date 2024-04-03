version := `awk '/^\[package\]/ {p=1; next} /^\[/ {p=0} p && /version\s*=\s*"[^"]+"/ {gsub(/version\s*=\s*"|"$/, ""); print}' Cargo.toml`
engine := if `which docker &> /dev/null; echo $?` == "0" {"sudo docker"} else {"podman"}

watch:
    cargo leptos watch

build:
    cargo leptos build --release

oci-build:
    {{engine}} build -t music-rs:{{version}} .

oci-run:
    {{engine}} run --init --name music --rm -p 3000:3000 -v ./public/assets:/app/site/assets:ro,Z music-rs:{{version}}