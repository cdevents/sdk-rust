check:
	cargo hack check --each-feature

# no uncommitted changes on sdk (generated code)
check_no_uncommitted_changes_on_sdk:
	git diff --exit-code cdevents-sdk

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

clean:
	cargo clean

generate:
	cargo run -p generator -- --templates-dir "generator/templates" --jsonschemas "cdevents-specs/*/schemas/*.json" --dest "cdevents-sdk/src/generated"

test:
	cargo nextest run --all-features
	cargo test --doc

# buid_cdevents-specs:
# 	git submodule deinit -f --all
# 	git submodule init
# 	git submodule add -f https://github.com/cdevents/spec.git cdevents-specs/main
# 	git submodule add -f -b spec-v0.3 https://github.com/cdevents/spec.git cdevents-specs/spec-v0.3
# 	git submodule add -f -b spec-v0.4 https://github.com/cdevents/spec.git cdevents-specs/spec-v0.4
# 	git submodule update -f --rebase -- cdevents-specs/main

update_cdevents-specs:
	git submodule update --recursive --remote

.PHONY:
	generate \
	check check_no_uncommitted_changes_on_sdk \
	test \
	lint_cargo_fmt_check lint_cargo_deny lint_cargo_clippy lint_cargo_toml_files \
	clean
