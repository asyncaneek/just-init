
# Set shell for Windows OSs:
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

mod examples

default:
    @just --list --unsorted

build:
    cargo build

run:
    cargo run

test:
    cargo test

fix:
    cargo fmt
    cargo fix --allow-dirty --allow-staged

show-help:
    cargo run -- --help