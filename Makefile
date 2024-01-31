build_docs:
	cargo doc --no-deps
build_examples:
	cargo build --examples
build:
	cargo build
build_release:
	cargo build --release
build_release_%:
	cargo build --release --package $*
build_%:
	cargo build --package $*

check:
	cargo hack check --each-feature

check_no_std:
	cargo --version
	cargo check --target thumbv7em-none-eabihf -p ockam --no-default-features --features 'no_std alloc software_vault'
	# no_std example project
	cd examples/rust/example_projects/no_std
	cargo check --example hello

check_cargo_update:
	cargo --version
	# TODO: uncomment when tauri version is updated
	# rm -rf Cargo.lock
	# cargo update
	# cargo check

# no uncommitted changes on sdk (generated code)
check_no_uncommitted_changes_on_sdk:
	git diff --exit-code cdevents-sdk

lint: lint_cargo_fmt_check lint_cargo_deny lint_cargo_clippy

lint_cargo_fmt_check:
	cargo fmt --all -- --check

lint_cargo_deny:
	cargo deny --workspace --all-features \
		--exclude-dev \
		--exclude generator \
		check licenses advisories \
		--config=tools/cargo-deny/deny.toml

lint_cargo_clippy:
	cargo clippy --workspace --all-features --no-deps --all-targets -- --deny warnings

lint_cargo_toml_fmt_files:
	dprint fmt --config=tools/dprint/dprint.json

lint_cargo_toml_check_files:
	dprint check --config=tools/dprint/dprint.json

test_cargo_nextest:
	cargo nextest run --all-features

test_cargo_check:
	cargo hack check --each-feature --no-dev-deps

test_cargo_machete:
	cargo machete --with-metadata

clean:
	cargo clean

generate:
	cargo run -p generator -- --templates-dir "generator/templates" --jsonschema-dir "cdevents-spec/schemas" --dest "cdevents-sdk/src/generated"

test:
	cargo nextest run --all-features
	cargo test --doc

.PHONY:
	generate \
	check check_no_uncommitted_changes_on_sdk \
	test \
	lint lint_cargo_fmt_check lint_cargo_deny lint_cargo_clippy lint_cargo_toml_files lint_cargo_readme lint_cargo_readme_% lint_cargo_toml_files \
	clean clean_% very_clean format
