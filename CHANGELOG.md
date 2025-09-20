# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.12](https://github.com/WyvernIXTL/wuerfel-rs/compare/v0.1.11...v0.1.12) - 2025-09-20

### Added

- add shell completion generation

### Other

- remove basterdized homebrew tap bs

## [0.1.11](https://github.com/WyvernIXTL/wuerfel-rs/compare/v0.1.10...v0.1.11) - 2025-08-28

### Other

- minor change
- add npm to installation section in readme
- add homebrew and binstall installation instructions, add what prebuilts to expect

## [0.1.10](https://github.com/WyvernIXTL/wuerfel-rs/compare/v0.1.9...v0.1.10) - 2025-08-28

### Other

- fix missing npm release, add attestation

## [0.1.9](https://github.com/WyvernIXTL/wuerfel-rs/compare/v0.1.8...v0.1.9) - 2025-08-28

### Other

- add npm and homebrew releases
- add homepage (same as repo url)

## [0.1.8](https://github.com/WyvernIXTL/wuerfel-rs/compare/v0.1.7...v0.1.8) - 2025-08-28

### Added

- add zeroizing allocator
- move to osrng from stdrng for higher entropy with large passwords

### Other

- added note regarding zeroization of allocations in readme
- run integration tests on windows and macos as well
- remove zeroizing container whose values are likely copied

## [0.1.7](https://github.com/WyvernIXTL/wuerfel-rs/compare/v0.1.6...v0.1.7) - 2025-08-28

### Added

- secured password and zeroized password list

### Other

- add updated note regarding security
- removed security section from readme
- word list storage and key generation
- fixed wrong sentences and typos
- add installation instructions for scoop

## [0.1.6](https://github.com/WyvernIXTL/wuerfel-rs/compare/v0.1.5...v0.1.6) - 2025-08-27

### Other

- disable release-plz github release due to incompatibility with cargo dist

## [0.1.5](https://github.com/WyvernIXTL/wuerfel-rs/compare/v0.1.4...v0.1.5) - 2025-08-27

### Other

- remove arm for windows build as target due to incompatibliity of cargo dist and license-fetcher

## [0.1.4](https://github.com/WyvernIXTL/wuerfel-rs/compare/v0.1.3...v0.1.4) - 2025-08-27

### Other

- revert cargo dist workflow change

## [0.1.3](https://github.com/WyvernIXTL/wuerfel-rs/compare/v0.1.2...v0.1.3) - 2025-08-27

### Other

- fix dist not triggering after release

## [0.1.2](https://github.com/WyvernIXTL/wuerfel-rs/compare/v0.1.1...v0.1.2) - 2025-08-27

### Other

- Merge branch 'master' of https://github.com/WyvernIXTL/wuerfel-rs
- added not regarding where to find prebuilts
- add cargo dist releases

## [0.1.1](https://github.com/WyvernIXTL/wuerfel-rs/compare/v0.1.0...v0.1.1) - 2025-08-27

### Other

- add installation section
- add asciinema video and add bullet point to security section regarding zeroization
- added changelog
- add badges
