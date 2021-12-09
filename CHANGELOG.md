# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Changed

- Added build scripts for Heml charts and Docker images.

## [0.3.0] - 2021-11-12


### Changed
- `operator-rs` `0.3.0` → `0.4.0` ([#96]).
- `bundle.tar.gz` now written to `tmp` to avoid permission problems in container ([#96]).

### Fixed
- `Reporule` artifacts (in custom resource) to `Regorule` ([#96]).

[#96]: https://github.com/stackabletech/regorule-operator/pull/96

## [0.2.0] - 2021-10-28

### Changed
- `operator-rs` `0.2.2` → `0.3.0` ([#92]).
- using `operator-rs` reexports ([#92]).
- moved `wait_until_crds_present` to operator-binary ([#92]).

### Removed
- `kube-rs` ([#92]).
- `k8s-openapi` ([#92]).

[#92]: https://github.com/stackabletech/regorule-operator/pull/92

## [0.1.0] - 2021-09-14

### Changed
- **Breaking:** Repository structure was changed and the -server crate renamed to -binary. As part of this change the -server suffix was removed from both the package name for os packages and the name of the executable ([#63]).
 
### Added

- Initial release

[#63]: https://github.com/stackabletech/regorule-operator/pull/63
