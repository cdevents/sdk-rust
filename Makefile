build_docs:
	cargo doc --locked --no-deps
build_examples:
	cargo --locked build --examples
build:
	cargo build --locked
build_release:
	cargo --locked build --release
build_release_%:
	cargo --locked build --release --package $*
build_%:
	cargo build --locked --package $*

check:
	cargo check --locked
check_no_std:
	cargo --version
	cargo check --locked --target thumbv7em-none-eabihf -p ockam --no-default-features --features 'no_std alloc software_vault'
	# no_std example project
	cd examples/rust/example_projects/no_std
	cargo check --example hello
check_cargo_update:
	cargo --version
	# TODO: uncomment when tauri version is updated
	# rm -rf Cargo.lock
	# cargo update
	# cargo check --locked

.PHONY:
	check \
	lint lint_cargo_fmt_check lint_cargo_deny lint_cargo_clippy lint_cargo_toml_files lint_cargo_readme lint_cargo_readme_% lint_cargo_toml_files \
	clean clean_% very_clean format
