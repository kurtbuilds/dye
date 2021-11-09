set dotenv-load := false

help:
    @just --list --unsorted

build:
    cargo build
alias b := build

run *args:
    cargo run {{args}}
alias r := run

release:
    cargo build --release

install:
    cargo install --path .

bootstrap:
    cargo install cargo-bump

test:
    cargo test

check:
    cargo check

# Bump version. level=major,minor,patch
bump level:
    git diff-index --exit-code HEAD > /dev/null || (echo You have untracked changes. Commit your changes before bumping the version. && exit 1)
    cargo bump {{level}}
    sleep 1 # I'm so confused, but cargo-bump seems to have a race condition where Cargo.lock doesn't get committed, so this fixes that.
    git commit -am "Bump {{level}} version"
    git tag v$(rg  "version = \"([0-9.]+)\"" -r '$1' Cargo.toml)
    git push origin v$(rg  "version = \"([0-9.]+)\"" -r '$1' Cargo.toml)

publish:
    cargo publish