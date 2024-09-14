# Set shell for Windows OSs:
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default:
    just --summary --unsorted

build:
    cargo build

run:
    cargo run

fix:
    cargo fmt
    cargo fix --allow-dirty --allow-staged