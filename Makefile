lint: lint_cargo_fmt_check lint_cargo_deny lint_cargo_clippy

lint_cargo_fmt_check:
	cargo fmt --all -- --check

lint_cargo_deny:
	cargo deny --all-features \
		check licenses advisories \
		--config=tools/cargo-deny/deny.toml

lint_cargo_clippy:
	cargo clippy --no-deps --all-targets -- -D warnings

lint_cargo_toml_fmt_files:
	dprint fmt --config=tools/dprint/dprint.json

lint_cargo_toml_check_files:
	dprint check --config=tools/dprint/dprint.json

clean:
	cargo clean

.PHONY:
	check \
	lint lint_cargo_fmt_check lint_cargo_deny lint_cargo_clippy lint_cargo_toml_files lint_cargo_readme lint_cargo_readme_% lint_cargo_toml_files \
	clean clean_% very_clean format
