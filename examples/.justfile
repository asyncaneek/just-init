# Set shell for Windows OSs:
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default:
    @just --list --unsorted

all: single-file single-path

single-file:
    cargo run -- --data single_file/data.json --source single_file/input --output single_file/out

single-path:
    cargo run -- --data single_path/data.json --source single_path/input --output single_path/out

single-inline:
    cargo run -- --inline-data "project_name=CppProject" --source cpp_project/input --output cpp_project/out

