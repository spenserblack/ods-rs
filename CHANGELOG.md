# Changelog

## [Unreleased](https://github.com/spenserblack/one-d-six-rs/compare/v0.2.1...master)

## [0.2.1] 2019/08/15
### Removed
- Accidental `pub use` of types from the `rand` crate

## [0.2.0] 2019/08/15
### Added
- `Rollable` trait that allows user to make a type that can be rolled for

### Changed
- All types in `one_d_six` to be usable for any type that implements `Rollable`, not just `u32`

## 0.1.0 2019/08/12
### Added
- Library for rolling dice
- Binary w/ CLI for rolling dice

[0.2.1]: https://github.com/spenserblack/one-d-six-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/spenserblack/one-d-six-rs/compare/v0.1.0...v0.2.0
