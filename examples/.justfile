# Set shell as nushell:
set shell := ["nu", "-c"]

default:
    @just --list --unsorted

all: single-file single-path

single-file:
    cargo run -- --data single_file/data.json --source single_file/input --output single_file/out

single-path:
    cargo run -- --data single_path/data.json --source single_path/input --output single_path/out
