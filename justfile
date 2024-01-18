default:
    @just --list --unsorted

_install_cargo-binstall:
    # cargo install cargo-binstall
    cargo binstall -V || curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

_install_cargo-nextest: _install_cargo-binstall
    cargo binstall cargo-nextest -y

_install_cargo-insta: _install_cargo-binstall
    cargo binstall cargo-insta -y

_install_cargo-release: _install_cargo-binstall
    cargo binstall cargo-release -y

_install_cargo-hack: _install_cargo-binstall
    cargo binstall cargo-hack -y

_install_cargo-deny: _install_cargo-binstall
    cargo binstall cargo-deny -y

_install_git-cliff: _install_cargo-binstall
    cargo binstall git-cliff -y

check: _install_cargo-hack
    cargo hack check --each-feature --no-dev-deps

generate:
    # cd generator; cargo run
    cargo run -p generator -- --templates-dir "generator/templates" --jsonschema-dir "cdevents-spec/schemas" --dest "cdevents-sdk/src/generated"

build:
    cargo build

alias fmt := format

# Format the code and sort dependencies
format:
    cargo fmt
    # cargo sort --workspace --grouped
    just --unstable --fmt

deny: _install_cargo-deny
    cargo deny check advisories
    cargo deny check bans licenses sources

# Lint all the code (megalinter + lint_rust)
lint: lint_rust megalinter

# Lint the rust code
lint_rust:
    just --unstable --fmt --check
    cargo fmt --all -- --check # generated code is not formatted
    # cargo sort --workspace --grouped --check
    cargo clippy --workspace --all-features --all-targets -- --deny warnings --allow deprecated --allow unknown-lints

# Lint with megalinter (locally via docker)
megalinter:
    # rm -rf megalinter-reports
    docker run --rm --name megalinter -it --env "DEFAULT_WORKSPACE=/tmp/lint" -v "${DOCKER_HOST:-/var/run/docker.sock}:/var/run/docker.sock:rw" -v "$PWD:/tmp/lint:rw" "oxsecurity/megalinter:v7"

# Launch tests
test: _install_cargo-nextest
    cargo nextest run
    # cargo test --doc
    # cargo hack nextest --each-feature -- --test-threads=1

changelog: _install_git-cliff
    git-cliff -o "CHANGELOG.md"
    git add CHANGELOG.md && git commit -m "📝 update CHANGELOG"

release *arguments: _install_cargo-release _install_git-cliff
    cargo release --workspace --execute {{ arguments }}
    # git-cliff could not be used as `pre-release-hook` of cargo-release because it uses tag
    git-cliff -o "CHANGELOG.md"
    git add CHANGELOG.md && git commit -m "📝 update CHANGELOG" && git push
