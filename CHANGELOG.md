# Changelog

[Keep a Changelog]:    https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][], and this project adheres to
[Semantic Versioning][].

## 0.8.0 (03 June 2025)

### Added

  - Added new lints available in Rust 1.83
      - `clippy::non_zero_suggestions`: `allow` -> `deny`
      - `clippy::unused_trait_names`: `allow` -> `warn`
      - `clippy::zombie_processes`: `warn` -> `deny`
  - Added new lints available in Rust 1.84
      - `clippy::map_with_unused_argument_over_ranges`: `allow` -> `warn`
      - `clippy::regex_creation_in_loops`: `warn` -> `deny`
  - Added new lints available in Rust 1.85
      - `clippy::as_pointer_underscore`: `allow` -> `warn`
      - `clippy::doc_include_without_cfg`: `allow` -> `deny`
  - Added new lints available in Rust 1.86
      - `clippy::doc_overindented_list_items`: `warn` -> `allow`
      - `clippy::mutex_integer`: `allow` -> `deny`
      - `clippy::precedence_bits`: `allow` -> `warn`
      - `clippy::return_and_then`: `allow` -> `warn`
  - Added Rust 2024 edition compatibility lints

### Changed

  - Upgraded lints:
      - `clippy::missing_assert_message`: `warn` -> `deny`


## 0.7.0 (30 October 2024)

### Added

  - Added new lints available in Rust 1.82
      - `clippy::pathbuf_init_then_push`: `allow` -> `warn`
      - `clippy::unused_result_ok`: `allow` -> `warn`

### Changed

  - Adjusted lint exceptions for tests


## 0.6.0 (05 September 2024)

### Added

  - Added new lints available in Rust 1.81
      - `clippy::allow_attributes`: `allow` -> `warn`
      - `clippy::field_scoped_visibility_modifiers`: `allow` -> `warn`

### Changed

  - Downgraded lints:
      - `clippy::tests_outside_test_module lint`: `forbid` -> `deny`
  - Adjusted lint overrides applied to tests

### Removed

  - Removed the `reasons` feature now that `lint_reasons` is stable


## 0.5.0 (31 August 2024)

### Added

  - Added new lints available in Rust 1.78
      - `clippy::deprecated_clippy_cfg_attr`: `warn` -> `deny`
      - `clippy::incompatible_msrv`: `warn` -> `deny`
      - `clippy::multiple_bound_locations`: `warn` -> `deny`
      - `clippy::unnecessary_clippy_cfg`: `warn` -> `deny`
  - Added new lints available in Rust 1.79
      - `clippy::const_is_empty`: `warn` -> `deny`
  - Added new lints available in Rust 1.80
      - `clippy::renamed_function_params`: `allow` -> `deny`


### Changed

  - Downgraded lints:
      - `clippy::missing_assert_message`: `forbid` -> `deny`
  - Updated README:
      - Updated IDEs section
      - Reformatted lints to link to official docs
      - Updated details of `lint_reasons`
  - Moved linting configuration to `Cargo.toml`
  - Added minimum Rust version to `Cargo.toml`
  - Updated release profile optimisations
  - Added example `rustfmt.toml`


## 0.4.0 (02 April 2024)

### Added

  - Added new lints available in Rust 1.77
      - `clippy::pub_underscore_fields`: `allow` -> `deny`
      - `clippy::empty_enum_variants_with_brackets`: `allow` -> `deny`
      - `clippy::unconditional_recursion`: `warn` -> `deny`

### Removed

  - Removed lints deprecated in Rust 1.77
      - `unused_tuple_struct_fields`


## 0.3.0 (10 March 2024)

### Added

  - Added new lints available in Rust 1.76
      - `clippy::infinite_loop`: `allow` -> `forbid`
      - `clippy::iter_over_hash_type`: `allow` -> `warn`
  - Added specific style for `impl` section header comments

### Changed

  - Downgraded lints:
      - `clippy::unwrap_in_result`: `forbid` -> `deny`


## 0.2.0 (27 November 2023)

### Added

  - Added a table of contents
  - Added recommended linting configuration with documentation
  - Added an approach for enforcing lint reasons
  - Added additional code to illustrate more example functionality


## 0.1.2 (16 September 2023)

### Changed

  - Changed recommendation from CLion to RustRover


## 0.1.1 (15 June 2023)

### Added

  - Added illustrative screenshots
  - Added additional notes about readability

### Changed

  - Clarified rules for links in Markdown files


## 0.1.0 (05 June 2023)

### Added

  - Added a project README file documenting the coding standards in depth
  - Added an example application
  - Added settings for JetBrains IDEs


