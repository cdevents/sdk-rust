# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2](https://github.com/cdevents/sdk-rust/compare/cdevents-sdk-v0.1.1...cdevents-sdk-v0.1.2) - 2025-01-07

### Other

- *(deps)* update thiserror requirement from 1.0 to 2.0

## [0.1.1](https://github.com/cdevents/sdk-rust/compare/cdevents-sdk-v0.1.0...cdevents-sdk-v0.1.1) - 2024-09-30

### Other

- *(deps)* update fluent-uri requirement from 0.2 to 0.3
- *(deps)* update rstest requirement from 0.22 to 0.23

## [0.1.0](https://github.com/cdevents/sdk-rust/releases/tag/cdevents-sdk-v0.1.0) - 2024-09-01

### Added
- add support for cdevents 0.4 ([#24](https://github.com/cdevents/sdk-rust/pull/24))
- [**breaking**] add support for mulitple version of cdevents' specifications  ([#19](https://github.com/cdevents/sdk-rust/pull/19))
- *(testkit)* provide proptest 's arbitraies to generate samples
- string enum are defined as rust Enum
- enforce mutation to only create coherente cdevents (type match subject.content)
- introduce support for cloudevents
- add support for `PartialEq`, `Eq` on `CDEvent`, use wrapper for `Uri` and `UriReference`
- use more precise type for uri & datetime than String
- use the same Subject struct for every event and an enum on Content

### Fixed
- enforce non-empty string for `id` and hardcode mapping between context.type and subject.type
- use the `context.type` (instead of `$id`) to define the module name

### Other
- link README to top README
- *(deps)* update fluent-uri requirement from 0.1 to 0.2 ([#31](https://github.com/cdevents/sdk-rust/pull/31))
- *(deps)* update rstest requirement from 0.18 to 0.21 ([#27](https://github.com/cdevents/sdk-rust/pull/27))
- *(deps)* update boon requirement from 0.5 to 0.6 ([#26](https://github.com/cdevents/sdk-rust/pull/26))
- *(deps)* update proptest-derive requirement from 0.4 to 0.5 ([#29](https://github.com/cdevents/sdk-rust/pull/29))
- change clipy configuration to support workspace and features
- add test to validate against jsonschema: examples (from spec), generated samples
- reformat Cargo.toml
- update typo, remove commented code, move comments,... apply review suggestion
- setup ci & linters
- reformat
- apply some recommendation from clippy
- enable deny_unknown_fields
- *(deps)* remove useless "serde_with"
- replace generation of json object by named types for Subjects content
- rename crates `cdevents` into `cdevents-sdk`
