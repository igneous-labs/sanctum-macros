# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.3.0] - 2024-04-15

### Changed

- Turned on all doc tests
- Narrow `heck` dependency range to avoid compile errors

## [1.2.0] - 2023-12-14

### Added

- Allow `declare_program_keys!()` to take multiple seeds so that seeds longer than 32 bytes can be used. If multiple seeds are used, they're named `*_SEED_0`, `*_SEED_1`, etc.

## [1.1.0] - 2023-12-14

### Added

- `create_with_seed!()`

## [1.0.0] - 2023-12-11

Initial release
