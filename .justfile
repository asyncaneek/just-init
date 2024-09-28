# Set shell as nushell:
set shell := ["nu", "-c"]

mod examples

default:
    @just --list --unsorted

@system-info:
    cargo --version
    just --version
    let nu_version = nu --version; print $"nu ($nu_version)"
    rustup show 

build:
    cargo build

test:
    cargo test

fix:
    cargo fmt
    cargo fix --allow-dirty --allow-staged

package-target TARGET="aarch64-pc-windows-msvc": test
    cargo build --release --target {{TARGET}} --target-dir target
    mkdir release/{{TARGET}}
    cp target/{{TARGET}}/release/*.exe release/{{TARGET}} --force --verbose

package-target-all: 
    just package-target aarch64-pc-windows-msvc
    just package-target x86_64-pc-windows-msvc