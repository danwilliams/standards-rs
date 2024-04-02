# Changelog

[Keep a Changelog]:    https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][], and this project adheres to
[Semantic Versioning][].


## 0.4.0 (02 April 2024)

### Added

  - Added new lints available in Rust 1.77
      - `clippy::pub_underscore_fields`
      - `clippy::empty_enum_variants_with_brackets`
      - `clippy::unconditional_recursion`

### Removed

  - Removed lints deprecated in Rust 1.77
      - `unused_tuple_struct_fields`


## 0.3.0 (10 March 2024)

### Added

  - Added new lints available in Rust 1.76
      - `clippy::infinite_loop`
      - `clippy::iter_over_hash_type`
  - Added specific style for `impl` section header comments

### Changed

  - Downgraded the `clippy::unwrap_in_result` lint from `forbid` to `deny`


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


