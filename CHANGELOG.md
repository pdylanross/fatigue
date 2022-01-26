## 0.2.7 (2022-01-26)


- fix: added default-members to workspace to prevent publishing the test api
- chore: re-ran cargo sort
- chore: ran cargo sort
- feat: added environment variable overrides
- chore(deps): bump reqwest from 0.11.8 to 0.11.9
- Bumps [reqwest](https://github.com/seanmonstar/reqwest) from 0.11.8 to 0.11.9.
- [Release notes](https://github.com/seanmonstar/reqwest/releases)
- [Changelog](https://github.com/seanmonstar/reqwest/blob/master/CHANGELOG.md)
- [Commits](https://github.com/seanmonstar/reqwest/compare/v0.11.8...v0.11.9)
- ---
updated-dependencies:
- dependency-name: reqwest
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- chore(deps): bump serde_json from 1.0.74 to 1.0.78
- Bumps [serde_json](https://github.com/serde-rs/json) from 1.0.74 to 1.0.78.
- [Release notes](https://github.com/serde-rs/json/releases)
- [Commits](https://github.com/serde-rs/json/compare/v1.0.74...v1.0.78)
- ---
updated-dependencies:
- dependency-name: serde_json
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- ci: dont bump on dependabot updates
- ci(revert): move kodiak toml to repo root
- This reverts commit 86da4fd831202456e747d07fcf709d12b0fc5fea.
- ci: move kodiak toml to repo root
- ci: add doc and ci to cz schema pattern
- ci: update kodiak config
- ci: update cz schema validation to allow for scopes
- chore(deps): bump serde from 1.0.133 to 1.0.135
- Bumps [serde](https://github.com/serde-rs/serde) from 1.0.133 to 1.0.135.
- [Release notes](https://github.com/serde-rs/serde/releases)
- [Commits](https://github.com/serde-rs/serde/compare/v1.0.133...v1.0.135)
- ---
updated-dependencies:
- dependency-name: serde
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- doc: update intro sentence
- doc: fixed a weird sentence
- doc: first pass on readme
- added contributing guide (thanks atom for the baseline)
added code_of_conduct (thanks contributor covenant)
added bug and issue templates (thanks rust & atom for the baselines)
- ci: split publish into two discrete steps to avoid weird caching issues

## 0.2.6 (2022-01-11)


- feat: csv loading
- feat: added json context action
- ci: prevent publish when we aren't actually publishing
- doc: updated demo gif, totally thought those borders were transparent
- doc: added demo gif and readme skeleton

## 0.2.5 (2022-01-08)


- feat: calculate and display requests per second

## 0.2.4 (2022-01-08)


- chore: merge main
- fix: fixed release-crate job

## 0.2.3 (2022-01-08)


- fix: fixed a few non-async locks that should've been async

## 0.2.2 (2022-01-07)


- fix: added docs to a few methods
- ci: the thing was over there
- [skip-ci] Release 0.2.1
- ci: use dashless skipci tag

## 0.2.1 (2022-01-07)


- ci: missed toolchain in bump
- fix: addressed clippy warnings
- ci: added toolchain back
- ci: added lint to main ci action
- ci: separate cargo releases from version bumping to allow for ci/doc commits to not fail actions
- ci: update cz rules for pre-1.0
- ci: update cz toml version format

## v0.2.0 (2022-01-07)

## v0.1.0 (2022-01-07)

### Feat

- basic minimal implementation

## v0.0.3 (2021-12-22)

### Fix

- updated descriptions and readmes

## v0.0.2 (2021-12-22)

### Fix

- fixed commitizen toml

## v0.0.1 (2021-12-22)

### Feat

- initial project layout
