# Set shell for Windows OSs:
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default:
    @just --list --unsorted

build:
    cargo build

run:
    cargo run

test:
    cargo test --quiet

fix:
    cargo fmt
    cargo fix --allow-dirty --allow-staged

show-help:
    cargo run -- --help