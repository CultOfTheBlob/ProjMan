default: run

run:
  @cargo run

fmt:
  @cargo fmt

lint: fmt
  @cargo clippy -- -D warnings

test: lint
  @cargo test

build: test
  @nix build

push: build
  @git push
