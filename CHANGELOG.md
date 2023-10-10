## 0.2.14 (2023-10-10)

## 0.2.13 (2023-09-01)


- fix: missed json output formatter file
- nbd, its not used anyways

## 0.2.12 (2023-09-01)


- ci: remove extraneous newline
- ci: should_publish should be the id
- ci: add -ex to should publish step
- chore: Bump actions/checkout from 2 to 3 (#187)
- Bumps actions/checkout from 2 to 3.
- Release notes
Sourced from actions/checkout's releases.
- v3.0.0
- Updated to the node16 runtime by default
- This requires a minimum Actions Runner version of v2.285.0 to run, which is by default available in GHES 3.4 or later.
- v2.7.0
What's Changed
- Add new public key for known_hosts (#1237) by @​TingluoHuang in actions/checkout#1238
- Full Changelog: actions/checkout@v2.6.0...v2.7.0
v2.6.0
What's Changed
- Add backports to v2 branch by @​cory-miller in actions/checkout#1040
- Includes backports from the following changes: actions/checkout#964, actions/checkout#1002, actions/checkout#1029
Upgraded the licensed version to match what is used in v3.
- Full Changelog: actions/checkout@v2.5.0...v2.6.0
v2.5.0
What's Changed
- Update @​actions/core to 1.10.0 by @​rentziass in actions/checkout#962
- Full Changelog: actions/checkout@v2...v2.5.0
v2.4.2
What's Changed
- Add set-safe-directory input to allow customers to take control. (#770) by @​TingluoHuang in actions/checkout#776
Prepare changelog for v2.4.2. by @​TingluoHuang in actions/checkout#778
- Full Changelog: actions/checkout@v2...v2.4.2
v2.4.1
- Fixed an issue where checkout failed to run in container jobs due to the new git setting safe.directory
- v2.4.0
- Convert SSH URLs like org-<ORG_ID>@github.com: to https://github.com/ - pr
- v2.3.5
Update dependencies
v2.3.4
- Add missing awaits
Swap to Environment Files
- v2.3.3
- Remove Unneeded commit information from build logs
Add Licensed to verify third party dependencies
- ... (truncated)
- 
Changelog
Sourced from actions/checkout's changelog.
- Changelog
v3.6.0
- Fix: Mark test scripts with Bash'isms to be run via Bash
Add option to fetch tags even if fetch-depth > 0
- v3.5.3
- Fix: Checkout fail in self-hosted runners when faulty submodule are checked-in
Fix typos found by codespell
Add support for sparse checkouts
- v3.5.2
- Fix api endpoint for GHES
- v3.5.1
- Fix slow checkout on Windows
- v3.5.0
- Add new public key for known_hosts
- v3.4.0
- Upgrade codeql actions to v2
Upgrade dependencies
Upgrade @​actions/io
- v3.3.0
- Implement branch list using callbacks from exec function
Add in explicit reference to private checkout options
[Fix comment typos (that got added in #770)](actions/checkout#1057)
- v3.2.0
- Add GitHub Action to perform release
Fix status badge
Replace datadog/squid with ubuntu/squid Docker image
Wrap pipeline commands for submoduleForeach in quotes
Update @​actions/io to 1.1.2
Upgrading version to 3.2.0
- v3.1.0
- Use @​actions/core saveState and getState
Add github-server-url input
- v3.0.2
- Add input set-safe-directory
- v3.0.1
- Fixed an issue where checkout failed to run in container jobs due to the new git setting safe.directory
Bumped various npm package versions
- v3.0.0
- 
... (truncated)
- 
Commits
- f43a0e5 Release 3.6.0 (#1437)
7739b9b Add option to fetch tags even if fetch-depth > 0 (#579)
96f5310 Mark test scripts with Bash'isms to be run via Bash (#1377)
c85c95e Release v3.5.3 (#1376)
d106d46 Add support for sparse checkouts (#1369)
f095bcc Fix typos found by codespell (#1287)
47fbe2d Fix: Checkout fail in self-hosted runners when faulty submodule are checked-i...
8e5e7e5 Release v3.5.2 (#1291)
eb35239 Fix: convert baseUrl to serverApiUrl 'formatted' (#1289)
83b7061 Release v3.5.1 (#1284)
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump katyo/publish-crates from 1 to 2 (#189)
- Bumps katyo/publish-crates from 1 to 2.
- Release notes
Sourced from katyo/publish-crates's releases.
- Release v2
Second release
- Commits
- e83e72a Added dist files
18ee0da Update dependencies
1aac5fa Update dependencies
d2d4dd4 Use exit code to detect errors in cargo metadata
8ffc98c Update readme with suggested action for PRs
3b2f17b Fix type mismatch in graphql query
95ccbb5 Get GitObject from sha rather than ref
31c25c4 Update dependencies
bc0ffd0 Update dependencies
525296d Update from crates.io, but don't touch the lock file
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump commitizen-tools/commitizen-action from 0.18.0 to 0.18.2 (#188)
- Bumps commitizen-tools/commitizen-action from 0.18.0 to 0.18.2.
- Release notes
Sourced from commitizen-tools/commitizen-action's releases.
- 0.18.2 (2023-05-18)
Fix
- install git-lfs into the dockerfile
- [master 4498afe] bump: version 0.18.1 → 0.18.2
2 files changed, 7 insertions(+), 1 deletion(-)
0.18.1 (2023-03-28)
Fix
- Add support for GitHub Enterprise Server (#66)
- [master e41bf7f] bump: version 0.18.0 → 0.18.1
2 files changed, 7 insertions(+), 1 deletion(-)
- Changelog
Sourced from commitizen-tools/commitizen-action's changelog.
- 0.18.2 (2023-05-18)
Fix
- install git-lfs into the dockerfile
- 0.18.1 (2023-03-28)
Fix
- Add support for GitHub Enterprise Server (#66)
- 
Commits
- 4498afe bump: version 0.18.1 → 0.18.2
f8f96fa fix: install git-lfs into the dockerfile
e41bf7f bump: version 0.18.0 → 0.18.1
468582c fix: Add support for GitHub Enterprise Server (#66)
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- ci: automerge GHA dependabot updates
- ci: add GHA to dependabot
- fix: bump tokio to the latest version
- This should fix the windows build issues.
- chore: Bump url from 2.4.0 to 2.4.1 (#186)
- Bumps url from 2.4.0 to 2.4.1.
- Commits
- a08aa2c Fix panic in set_path for file URLs  (#865)
edabc79 Fix no_std for idna (#843)
1158370 Update WPT data and expectations (#859)
beb2cde Stabilize debugger_visualizer feature (#855)
86730f1 Add --generate-link-to-definition option when building on docs.rs (#858)
e4dbb43 Implement std::error::Error for InvalidBase64 (#856)
b33514a Rewrite WPT runner (#857)
b228574 Enable the GitHub merge queue (#851)
3a474c3 Merge pull request #698 from lucacasonato/impl_error_for_data_url
1f7dbe0 Merge pull request #848 from lucacasonato/fix_trailing_space_pathname_setter
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump reqwest from 0.11.19 to 0.11.20 (#184)
- ⚠️  Dependabot is rebasing this PR ⚠️
Rebasing might not happen immediately, so don't worry if this takes some time.
Note: if you make any changes to this PR yourself, they will take precedence over the rebase.
- Bumps reqwest from 0.11.19 to 0.11.20.
- Release notes
Sourced from reqwest's releases.
- v0.11.20
What's Changed
- Fix deflate decompression back to using zlib, as outlined in the spec.
- 
Changelog
Sourced from reqwest's changelog.
- v0.11.20
- Fix deflate decompression back to using zlib, as outlined in the spec.
- 
Commits
- a4e8ab6 v0.11.20
84eb650 Revert "Fix deflate decompression (#1927)" (#1952)
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.186 to 1.0.188 (#185)
- Bumps serde from 1.0.186 to 1.0.188.
- Release notes
Sourced from serde's releases.
- v1.0.188
- Fix "failed to parse manifest" error when building serde using a Cargo version between 1.45 and 1.50 (#2603)
- v1.0.187
- Remove support for Emscripten targets on rustc older than 1.40 (#2600)
- 
Commits
- dad15b9 Release 1.0.188
d89c19f Revert "Adopt new Cargo feature resolver"
146dc0f Release 1.0.187
d26852d Merge pull request #2602 from dtolnay/resolver
e1c2724 Adopt new Cargo feature resolver
dbbfe7a Merge pull request #2600 from dtolnay/oldemscripten
dc24d12 Clean up all usage of serde_if_integer128
4e7533e Remove support for emscripten targets on rustc older than 1.40
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.185 to 1.0.186 (#183)
- Bumps serde from 1.0.185 to 1.0.186.
- Release notes
Sourced from serde's releases.
- v1.0.186
- Disallow incompatible versions of serde_derive and serde in the dependency graph (#2588, thanks @​soqb)
- 
Commits
- 5d03651 Release 1.0.186
a741293 Merge pull request #2599 from dtolnay/encodeutf8
f8d0b26 Remove custom encode_utf8 implementation in favor of standard one
7007c1b Merge pull request #2598 from dtolnay/lockstepversion
0d8ebac Duplicate the serde_derive dependency version in one fewer place
212c42c Merge pull request #2597 from dtolnay/lockstepcomment
919f6be Reword PR 2588 comment
c0f7042 Merge pull request 2588 from soqb:use-impossible-cfg-to-lockstep-serde-derive
e797c90 Merge pull request #2594 from Uzaaft/master
fc04d12 Remove useless discard of function argument that is used
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump tokio from 1.16.1 to 1.20.5 (#182)
- Bumps tokio from 1.16.1 to 1.20.5.
- Release notes
Sourced from tokio's releases.
- Tokio v1.20.5
1.20.5 (May 28, 2023)
Forward ports 1.18.6 changes.
Fixed
- deps: disable default features for mio (#5728)
- #5728: tokio-rs/tokio#5728
Tokio v1.20.2
1.20.2 (September 27, 2022)
This release removes the dependency on the once_cell crate to restore the MSRV of the 1.20.x LTS release. (#5048)
#5048: tokio-rs/tokio#5048
Tokio v1.20.1
1.20.1 (July 25, 2022)
Fixed
- chore: fix version detection in build script (#4860)
- #4860: tokio-rs/tokio#4860
Tokio v1.20.0
1.20.0 (July 12, 2022)
Added
- tokio: add track_caller to public APIs (#4772, #4791, #4793, #4806, #4808)
sync: Add has_changed method to watch::Ref (#4758)
- Changed
- time: remove src/time/driver/wheel/stack.rs (#4766)
rt: clean up arguments passed to basic scheduler (#4767)
net: be more specific about winapi features (#4764)
tokio: use const initialized thread locals where possible (#4677)
task: various small improvements to LocalKey (#4795)
- Fixed
Documented
- fs: warn about performance pitfall (#4762)
chore: fix spelling (#4769)
sync: document spurious failures in oneshot (#4777)
sync: add warning for watch in non-Send futures (#4741)
chore: fix typo (#4798)
- ... (truncated)
- 
Commits
- edd172c chore: prepare Tokio v1.20.5 (#5731)
9877fa2 Merge 'tokio-1.18.6' into 'tokio-1.20.x' (#5730)
0f898a3 chore: prepare Tokio v1.18.6 (#5729)
d6a9ef5 tokio: disable default features for mio (#5728)
2a18018 ci: fix CI for 1.18.x branch (#5728)
f3ce29a chore: prepare Tokio v1.20.4 release
0d8fe5f Merge branch 'tokio-1.18.x' into tokio-1.20.x
171ce0f chore: prepare Tokio v1.18.5 release
d6ea7a7 Add T: Unpin bound to ReadHalf::unsplit
ba81945 chore: prepare Tokio 1.20.3 release
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump reqwest from 0.11.18 to 0.11.19 (#181)
- Bumps reqwest from 0.11.18 to 0.11.19.
- Release notes
Sourced from reqwest's releases.
- v0.11.19
What's Changed
- Add ClientBuilder::http1_ignore_invalid_headers_in_responses() option.
Add ClientBuilder::http1_allow_spaces_after_header_name_in_responses() option.
Add support for ALL_PROXY environment variable.
Add support for use_preconfigured_tls when combined with HTTP/3.
Fix deflate decompression from using the zlib decoder.
Fix Response::{text, text_with_charset}() to strip BOM characters.
Fix a panic when HTTP/3 is used if UDP isn't able to connect.
Fix some dependencies for HTTP/3.
Increase MSRV to 1.63.
- New Contributors
- @​nyurik made their first contribution in seanmonstar/reqwest#1849
@​smndtrl made their first contribution in seanmonstar/reqwest#1856
@​attila-lin made their first contribution in seanmonstar/reqwest#1869
@​ollyswanson made their first contribution in seanmonstar/reqwest#1898
@​VivekPanyam made their first contribution in seanmonstar/reqwest#1903
@​bouzuya made their first contribution in seanmonstar/reqwest#1922
@​cipherbrain made their first contribution in seanmonstar/reqwest#1927
@​T-Sujeeban made their first contribution in seanmonstar/reqwest#1926
@​eric-seppanen made their first contribution in seanmonstar/reqwest#1852
- 
Changelog
Sourced from reqwest's changelog.
- v0.11.19
- Add ClientBuilder::http1_ignore_invalid_headers_in_responses() option.
Add ClientBuilder::http1_allow_spaces_after_header_name_in_responses() option.
Add support for ALL_PROXY environment variable.
Add support for use_preconfigured_tls when combined with HTTP/3.
Fix deflate decompression from using the zlib decoder.
Fix Response::{text, text_with_charset}() to strip BOM characters.
Fix a panic when HTTP/3 is used if UDP isn't able to connect.
Fix some dependencies for HTTP/3.
Increase MSRV to 1.63.
- 
Commits
- 8b49fc9 v0.11.19
87ff5d9 improve error message if incompabitle Identity with selected backend (#1852)
42f57b4 Fix panic in building h3 client when udp is forbidden (#1945)
4aa8516 msrv: bump to 1.63 (#1947)
1f6c2cf Add ClientBuilder::http1_allow_spaces_after_header_name_in_responses() (#1932)
6f07b9f Add ClientBuilder::http1_ignore_invalid_headers_in_responses() (#1926)
8396233 Fix deflate decompression (#1927)
b0c07a2 Bump rustls to v0.21.6 (#1928)
99bbae6 Bump webpki-roots to v0.25 (#1922)
61b1b2b Bump wasm-streams dependency to 0.3 (#1903)
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.183 to 1.0.185 (#180)
- Bumps serde from 1.0.183 to 1.0.185.
- Release notes
Sourced from serde's releases.
- v1.0.185
- Fix error "cannot move out of *self which is behind a shared reference" deriving Serialize on a non_exhaustive enum (#2591)
- v1.0.184
- Restore from-source serde_derive build on all platforms — eventually we'd like to use a first-class precompiled macro if such a thing becomes supported by cargo / crates.io
- 
Commits
- 3c7dd6f Release 1.0.185
8b196ea Merge pull request #2592 from dtolnay/remotenonexhaustive
1f8c8ad Fix "cannot move out of *self which is behind a shared reference"
870925d Add repro of issue 2591
d593215 No need for slow macOS CI if there is no platform-specific code
110af31 Merge pull request #2590 from pinkforest/phase-out-precompiled
360606b Following consensus on: #2580 (review)
151b45a Release 1.0.184
4617c95 Merge pull request #2587 from wucke13/master
2547ed8 fix shebang in build.sh
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde_json from 1.0.104 to 1.0.105 (#178)
- ⚠️  Dependabot is rebasing this PR ⚠️
Rebasing might not happen immediately, so don't worry if this takes some time.
Note: if you make any changes to this PR yourself, they will take precedence over the rebase.
- Bumps serde_json from 1.0.104 to 1.0.105.
- Release notes
Sourced from serde_json's releases.
- v1.0.105
- Support bool in map keys (#1054)
- 
Commits
- 0daacdd Release 1.0.105
59d9f96 Merge pull request #1055 from dtolnay/boolkey
9b69f16 Delete test_serialize_rejects_bool_keys
68a5582 Support deserializing bool in map keys
283a68b Support serializing bool in map keys
58dd8d9 Add test for boolean keys in map
8652bf2 Resolve ignored_unit_patterns pedantic clippy lint
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump thiserror from 1.0.46 to 1.0.47 (#179)
- Bumps thiserror from 1.0.46 to 1.0.47.
- Release notes
Sourced from thiserror's releases.
- 1.0.47
- Work around rust-analyzer bug (rust-lang/rust-analyzer#9911)
- 
Commits
- 0495eaa Release 1.0.47
2d9425c Work around ridiculous rust-analyzer behavior
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump thiserror from 1.0.45 to 1.0.46 (#177)
- Bumps thiserror from 1.0.45 to 1.0.46.
- Release notes
Sourced from thiserror's releases.
- 1.0.46
- Add bootstrap workaround to allow rustc to depend on thiserror (#248, thanks @​RalfJung)
- 
Commits
- 5ada5d5 Release 1.0.46
f51271a Reword bootstrap comment
1f02cdf Merge pull request #248 from RalfJung/bootstrap
fa63782 don't run build probes in rustc bootstrap
2fd79cd Merge pull request #247 from dtolnay/errorprovide
78e0ffe Pull in anyhow's new Error::provide support
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump thiserror from 1.0.44 to 1.0.45 (#176)
- Bumps thiserror from 1.0.44 to 1.0.45.
- Release notes
Sourced from thiserror's releases.
- 1.0.45
- Update backtrace support to nightly's new Error::provide API (rust-lang/rust#113464, #246)
- 
Commits
- 06f1895 Release 1.0.45
a11330f Merge pull request #246 from dtolnay/errorprovide
8a95c25 Update to nightly's new Error::provide API
543e123 Revert "Temporarily disable -Zrandomize-layout due to rustc ICE"
41b23f2 Temporarily disable -Zrandomize-layout due to rustc ICE
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump async-trait from 0.1.72 to 0.1.73 (#175)
- Bumps async-trait from 0.1.72 to 0.1.73.
- Release notes
Sourced from async-trait's releases.
- 0.1.73
- Prevent generated code from triggering ignored_unit_patterns pedantic clippy lint
- 
Commits
- f07c856 Release 0.1.73
f12f371 Resolve ignored_unit_patterns pedantic clippy lint in test suite
a71e066 Resolve ignored_unit_patterns pedantic clippy lint in generated code
7d4e192 Update ui test suite to nightly-2023-08-10
47565d9 Revert "Temporarily disable -Zrandomize-layout due to rustc ICE"
72bd72b Temporarily disable -Zrandomize-layout due to rustc ICE
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot show <dependency name> ignore conditions will show all of the ignore conditions of the specified dependency
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.181 to 1.0.183 (#174)
- Bumps serde from 1.0.181 to 1.0.183.
- Release notes
Sourced from serde's releases.
- v1.0.183
- Support deserializing Box<OsStr> with an equivalent representation as OsString (#2556, thanks @​DBLouis)
- v1.0.182
- Render field aliases in sorted order in error messages (#2458, thanks @​Mingun)
Support serde(default) on tuple structs (#2553, thanks @​Mingun)
- 
Commits
- 05a5b7e Release 1.0.183
3bff326 Merge pull request #2555 from Mingun/field
aaadd93 Merge pull request #2556 from DBLouis/master
9c864f0 Add forward impl for OsStr
070cce0 Get rid of temporary variable
b58e8ba Replace if let Some(...) = ... to Option::map
ada50b0 ignore_variant variable is always None, let's take this into account
5e313a7 Move generiс code out-of-function, create more specialized and simple code
2a36d11 Introduce a dedicated function for generating Field enum
b6685cf Release 1.0.182
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.180 to 1.0.181 (#173)
- Bumps serde from 1.0.180 to 1.0.181.
- Release notes
Sourced from serde's releases.
- v1.0.181
- Make serde(alias) work in combination with flatten when using in-place deserialization (#2443, thanks @​Mingun)
Improve the representation of adjacently tagged enums in formats where enum tags are serialized by index, as opposed to by string name (#2505, #2496, thanks @​Baptistemontan)
- 
Commits
- 57dc0ee Release 1.0.181
5e102c4 Relocate private size_hint module
4aa5422 Delete double reference when setting up adjacently tagged variant seed
ef4f860 Improve "expecting" message of adjacently tagged enum variant
9bd52ec Inline AdjacentlyTaggedEnumVariant::new
5cdd82d Remove Serializer from name of private type that is not a Serializer
110bf10 Condense AdjacentlyTaggedEnummVariantVisitor implementation
43035f6 Merge pull request #2505 from Baptistemontan/rework_adjacently_tagged_enum
83b1a3d Merge pull request #2443 from Mingun/deserialize-in-place
878110a Simplify code after dead code elimination
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.179 to 1.0.180 (#172)
- Bumps serde from 1.0.179 to 1.0.180.
- Release notes
Sourced from serde's releases.
- v1.0.180
- Update to 2018 edition
- 
Commits
- 033d05f Release 1.0.180
fe4e3fd Merge pull request #2547 from dtolnay/tombstone
8a8a8a7 Delete tombstones of the __private module
339dca8 Merge pull request #2546 from dtolnay/edition
0d7349f Resolve ambiguous core import on rustc 1.64 through 1.71
830528d Update to 2018 edition
ab90fbc Apply 'cargo fix --edition'
3eec111 Delete support for compilers without crate::-based module system
9388433 Rename 'try!' macro to 'tri!' in preparation for 2018 edition
ba12070 Merge pull request #2545 from dtolnay/up
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.177 to 1.0.179 (#171)
- Bumps serde from 1.0.177 to 1.0.179.
- Release notes
Sourced from serde's releases.
- v1.0.179
- Support serialization of tuple variants inside a flattened field (#2448, thanks @​Mingun)
- v1.0.178
- Fix build error when using serde with "std" feature turned off and "unstable" feature turned on (#2541)
- 
Commits
- c2b16bf Release 1.0.179
e7df537 Resolve doc_markdown clippy lint from PR 2448
02c34e4 Resolve redundant_field_names clippy lint from PR 2448
427c839 Merge pull request #2448 from Mingun/ser-flatten-enums
48aa054 Release 1.0.178
3616860 Delete broken symlink from precompiled derive sources
861b0df Consistently list StdError under 'Re-exports' heading of rustdoc
8b3d71a Merge pull request #2541 from dtolnay/de-core-error
ff5442c Add no-std unstable build in CI
92d686f Fix serde:🇩🇪:StdError in no-std unstable build
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.175 to 1.0.177 (#170)
- Bumps serde from 1.0.175 to 1.0.177.
- Release notes
Sourced from serde's releases.
- v1.0.177
- Add serde(rename_all_fields = "...") attribute to apply a rename_all on every struct variant of an enum (#1695, thanks @​jplatte)
Improve diagnostics for attribute parse errors (#2536, thanks @​jplatte)
- v1.0.176
- Allow tag field of an internally tagged enum to have same name as a field inside a skipped struct variant (#2266, thanks @​flisky)
- 
Commits
- 0676673 Release 1.0.177
7a4335d Merge pull request #2536 from jplatte/jplatte/error-span
31a0e73 Update error span for attribute / data kind mismatches
74fe708 Ignore return_self_not_must_use pedantic clippy lint
e74925b Merge pull request #1695 from jplatte/rename_all_fields
56be1c2 Pass RenameRule, RenameAllRules by value
2f9bf4d Add #[serde(rename_all_fields = "foo")] attribute
ad94aed Merge pull request #2535 from dtolnay/baretrait
30db83f Restore bare_trait_objects lint within serde_derive code
b0f7b00 Resolve manual_string_new pedantic clippy lint
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde_json from 1.0.103 to 1.0.104 (#168)
- Bumps serde_json from 1.0.103 to 1.0.104.
- Release notes
Sourced from serde_json's releases.
- v1.0.104
- Provide IntoDeserializer impl for &serde_json::Value (#1045, thanks @​ZetaNumbers)
- 
Commits
- ab08483 Release 1.0.104
f6cc4f3 Merge pull request #1045 from ZetaNumbers/value-ref-into-deserializer
8e8db8c Implement IntoDeserializer for &Value
8f90eac Delete inline attributes throughout test suite
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.174 to 1.0.175 (#167)
- Bumps serde from 1.0.174 to 1.0.175.
- Release notes
Sourced from serde's releases.
- v1.0.175
- Restore missing LICENSE files in serde_derive crate (#2527, thanks @​ankane)
- 
Commits
- 6140b6f Release 1.0.175
4cabc9f Merge pull request #2527 from ankane/license-files
aa7c634 Include license files in serde_derive crate
27414c9 Merge pull request #2522 from serde-rs/leak
50e2f4b Leak all memory allocated during macro expansion
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump thiserror from 1.0.43 to 1.0.44 (#166)
- ⚠️  Dependabot is rebasing this PR ⚠️
Rebasing might not happen immediately, so don't worry if this takes some time.
Note: if you make any changes to this PR yourself, they will take precedence over the rebase.
- Bumps thiserror from 1.0.43 to 1.0.44.
- Release notes
Sourced from thiserror's releases.
- 1.0.44
- Documentation improvements
- 
Commits
- 54b70cf Release 1.0.44
f86e8e5 Opt in to generate-link-to-definition when building on docs.rs
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde_yaml from 0.9.24 to 0.9.25 (#165)
- Bumps serde_yaml from 0.9.24 to 0.9.25.
- Release notes
Sourced from serde_yaml's releases.
- 0.9.25
- Serialize using quoted style around scalar that has digits with leading zero (#347)
- 
Commits
- f26dac4 Release 0.9.25
fda96c7 Merge pull request #383 from dtolnay/leadingzero
a38768f Quoted style for string consisting of digits with leading zero
cd1fd90 Add test of quoting of number with leading zeros
c1b1eac Resolve incorrect_partial_ord_impl_on_ord_type clippy lint
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump async-trait from 0.1.71 to 0.1.72 (#164)
- Bumps async-trait from 0.1.71 to 0.1.72.
- Release notes
Sourced from async-trait's releases.
- 0.1.72
- Documentation improvements
- 
Commits
- a01e5d4 Release 0.1.72
a38d35a Opt in to generate-link-to-definition when building on docs.rs
036a373 Update ui tests with 2021-edition diagnostics
059aafd Update to 2021 edition
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.173 to 1.0.174 (#163)
- Bumps serde from 1.0.173 to 1.0.174.
- Release notes
Sourced from serde's releases.
- v1.0.174
- Documentation improvements
- 
Commits
- 22be673 Release 1.0.174
166c89f Opt in to generate-link-to-definition when building on docs.rs
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.171 to 1.0.173 (#162)
- Bumps serde from 1.0.171 to 1.0.173.
- Release notes
Sourced from serde's releases.
- v1.0.173
- Fix missing trait implementations when using serde derive macro on a macro-generated data structure, such as via the bitflags crate (#2516)
- v1.0.172
- Experiment with precompiling the serde_derive macros to reduce build time (#2514)
- 
Commits
- 6e0b13e Release 1.0.173
7e8f978 Handle $crate special case
6c0e838 Always consider empty output to be unsuccessful
d3da419 Enable full expression parsing for precompiled serde_derive
425a4b7 Check precompiled subprocess exit status
63c65ef Release 1.0.172
e838b0b Release 1.0.172-alpha.0
041e99c Implement fallback to compiling serde_derive from source
07dcc4f Remove unneeded 'include' Cargo.toml entries
b88052d Rearrange precompiled directory
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde_yaml from 0.9.23 to 0.9.24 (#161)
- Bumps serde_yaml from 0.9.23 to 0.9.24.
- Release notes
Sourced from serde_yaml's releases.
- 0.9.24
- Implement FromStr for serde_yaml::Number (#381)
- 
Commits
- da99545 Release 0.9.24
053af6f Merge pull request #382 from dtolnay/parsenumber
3c68165 Add test of Number parsing
610d7b2 Implement FromStr for serde_yaml::Number
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde_json from 1.0.102 to 1.0.103 (#158)
- ⚠️  Dependabot is rebasing this PR ⚠️
Rebasing might not happen immediately, so don't worry if this takes some time.
Note: if you make any changes to this PR yourself, they will take precedence over the rebase.
- Bumps serde_json from 1.0.102 to 1.0.103.
- Release notes
Sourced from serde_json's releases.
- v1.0.103
- Documentation improvements
- 
Commits
- 54bcb4d Release 1.0.103
9c2879a Opt in to generate-link-to-definition when building on docs.rs
d1a07e2 Fix rustdoc::bare_urls lint in lexical code
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde_yaml from 0.9.22 to 0.9.23 (#160)
- Bumps serde_yaml from 0.9.22 to 0.9.23.
- Release notes
Sourced from serde_yaml's releases.
- 0.9.23
- Documentation improvements
- 
Commits
- 6b212e0 Release 0.9.23
cc83b0a Opt in to generate-link-to-definition when building on docs.rs
954e384 Make minimal-versions job consistent with other repos
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump uuid from 1.4.0 to 1.4.1 (#159)
- Bumps uuid from 1.4.0 to 1.4.1.
- Release notes
Sourced from uuid's releases.
- 1.4.1
What's Changed
- Fix macro hygiene by @​teohhanhui in uuid-rs/uuid#694
Add #[inline] for Uuid::from_bytes[_ref] and Uuid::{as,into}_bytes by @​jrose-signal in uuid-rs/uuid#693
Print uuids in examples by @​KodrAus in uuid-rs/uuid#697
Prepare for 1.4.1 release by @​KodrAus in uuid-rs/uuid#698
- New Contributors
- @​teohhanhui made their first contribution in uuid-rs/uuid#694
@​jrose-signal made their first contribution in uuid-rs/uuid#693
- Full Changelog: uuid-rs/uuid@1.4.0...1.4.1
- Commits
- 97b7f07 Merge pull request #698 from uuid-rs/cargo/1.4.1
8e930cf prepare for 1.4.1 release
0041b3b Merge pull request #697 from uuid-rs/chore/example-printing
5a1f3f5 use uuid_unstable
6b0cfb2 Merge pull request #693 from jrose-signal/inline-from_bytes
33f6b3e print uuids in examples
bd7df72 Merge pull request #694 from teohhanhui/fix/macro-hygiene
1d1ae31 Fix macro hygiene
317d925 Add #[inline] for Uuid::from_bytes[_ref] and Uuid::{as,into}_bytes
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.168 to 1.0.171 (#157)
- Bumps serde from 1.0.168 to 1.0.171.
- Release notes
Sourced from serde's releases.
- v1.0.171
- Support derive(Deserialize) on unit structs that have const generics (#2500, thanks @​Baptistemontan)
- v1.0.170
- Produce error message on suffixed string literals inside serde attributes (#2242)
Support single identifier as unbraced default value for const generic parameter (#2449)
- v1.0.169
- Add Deserializer::deserialize_identifier support for adjacently tagged enums (#2475, thanks @​Baptistemontan)
Fix unused_braces lint in generated Deserialize impl that uses braced const generic expressions (#2414)
- 
Commits
- 03da66c Release 1.0.171
f75426f Inline visitor_expr of unit struct deserialize impl
662fc38 Add test of const-generic unit struct where-clause edge case
28c1002 Merge pull request #2500 from Baptistemontan/derive_generic_unit_struct
89c8d85 allow Deserialize derive to handle generic unit structs
6502838 Release 1.0.170
c93a0f3 Merge pull request #2499 from dtolnay/strsuffix
8264e00 Reject suffixed string literals inside serde attrs
117ef22 Add ui test with suffixed string literals in attribute
3fb5e71 Release 1.0.169
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde_json from 1.0.100 to 1.0.102 (#156)
- Bumps serde_json from 1.0.100 to 1.0.102.
- Release notes
Sourced from serde_json's releases.
- v1.0.102
- Add a way to customize the serialization of byte arrays (#1039)
- v1.0.101
- Allow f32 and f64 as keys in maps (#1027, thanks @​overdrivenpotato)
- 
Commits
- 658689d Release 1.0.102
42dbd00 Merge pull request #1039 from dtolnay/writebytearray
a1ca32a Factor out byte array serialization to a new Formatter method
857b010 Inline Serializer::serialize_u8 into serialize_bytes
6ad5495 Inline u8::serialize into serialize_bytes
44b4a6c Simplify serialize_bytes
0e2c949 Inline SerializeSeq::end into serialize_bytes
1b72f2b Inline SerializeSeq::serialize_element into serialize_bytes
55a7f5c Inline Serializer::serialize_seq into serialize_bytes
3ddda75 Format PR 1037 with rustfmt
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.167 to 1.0.168 (#155)
- Bumps serde from 1.0.167 to 1.0.168.
- Release notes
Sourced from serde's releases.
- v1.0.168
- Allow serde::de::IgnoredAny to be the type for a serde(flatten) field (#2436, thanks @​Mingun)
Allow larger preallocated capacity for smaller elements (#2494)
- 
Commits
- 09b78b2 Release 1.0.168
a622b8a Merge pull request #2495 from dtolnay/cautious
399ef08 Allow larger preallocated capacity for smaller elements
3686277 Merge pull request #2436 from Mingun/flatten-ignored-any
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.166 to 1.0.167 (#154)
- Bumps serde from 1.0.166 to 1.0.167.
- Release notes
Sourced from serde's releases.
- v1.0.167
- Add serialize and deserialize impls for RangeFrom and RangeTo (#2471, thanks @​tbu-)
- 
Commits
- 807bd20 Release 1.0.167
ed9a140 Merge pull request #2444 from Mingun/dedup
2de7c2b Resolve redundant_static_lifetimes clippy lint from PR 2471
e6a4a37 Delete unuseful RangeFull impls
0fca04e Merge pull request 2471 from tbu-/pr_more_ranges
92bfc8d Merge pull request #2290 from Mingun/enum-tests-and-cleanup
fa0312a More formatting of doc tests and example code
1920b69 Declare required automod dev-dependency
3bfd41d Format doctests using rustfmt's format_code_in_doc_comments
290449f Fix doc tests to work whether or not serde derive feature is used
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump thiserror from 1.0.41 to 1.0.43 (#153)
- Bumps thiserror from 1.0.41 to 1.0.43.
- Release notes
Sourced from thiserror's releases.
- 1.0.42
- Fix compile error in derived Display impl if there was a nonstandard write! macro in scope (#239)
- 
Commits
- 225adab Release 1.0.43
f6dc5e5 Merge pull request #242 from dtolnay/stdwrite
cab9fec Avoid calling a nonstandard write! macro that might be in scope
900f018 Revert "Avoid calling a nonstandard write! macro that might be in scope"
305be4a Release 1.0.42
6165f58 Merge pull request #240 from dtolnay/stdwrite
264b7d1 Avoid calling a nonstandard write! macro that might be in scope
43f3a2a Update to 2021 edition
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump async-trait from 0.1.70 to 0.1.71 (#152)
- Bumps async-trait from 0.1.70 to 0.1.71.
- Release notes
Sourced from async-trait's releases.
- 0.1.71
- Documentation improvements
- 
Commits
- 11dfe16 Release 0.1.71
85172d3 Merge pull request #249 from erer1243/update-docs-lifetime
92acf81 Update doc comments to use lifetime 'async_trait instead of 'async
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde_json from 1.0.99 to 1.0.100 (#150)
- ⚠️  Dependabot is rebasing this PR ⚠️
Rebasing might not happen immediately, so don't worry if this takes some time.
Note: if you make any changes to this PR yourself, they will take precedence over the rebase.
- Bumps serde_json from 1.0.99 to 1.0.100.
- Release notes
Sourced from serde_json's releases.
- v1.0.100
- Support -Z minimal-versions
- 
Commits
- d2fce19 Release 1.0.100
897f913 No pre_ci in this repo
8f8a2b1 Eliminate syn 1 from minimal-versions
f482ed3 Add CI job using minimal-versions
79caf27 Sort Cargo.toml dependencies list
e98e664 Resolve explicit_iter_loop pedantic clippy lint
fdb7800 Support a manual trigger on CI workflow
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump async-trait from 0.1.69 to 0.1.70 (#151)
- Bumps async-trait from 0.1.69 to 0.1.70.
- Commits
- f8ce3fb Release 0.1.70
46631cc Eliminate syn 1 from minimal-versions
eb21940 Add CI job using minimal-versions
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump thiserror from 1.0.40 to 1.0.41 (#149)
- Bumps thiserror from 1.0.40 to 1.0.41.
- Commits
- 281997e Release 1.0.41
c28f8fa Eliminate syn 1 from minimal-versions
3cda68a Add CI job using minimal-versions
23d8fad Remove .clippy.toml in favor of respecting rust-version from Cargo.toml
5cc5855 Revert "Ui tests with compile_error resolved at call site"
b534daf Ui tests with compile_error resolved at call site
6735d4b Show error details during miri setup in CI
39aaeb0 Use error reporting provided by Meta
07c66e7 Run clippy on all crates in workspace
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.164 to 1.0.166 (#148)
- Bumps serde from 1.0.164 to 1.0.166.
- Release notes
Sourced from serde's releases.
- v1.0.166
- Add no-alloc category to crates.io metadata
- v1.0.165
- Fix incorrect count of fields passed to tuple deserialization methods when using serde(skip_deserializing) attributes (#2466, thanks @​Mingun)
Fix -Zminimal-versions build
- 
Commits
- 48479e4 Release 1.0.166
dfaf48b Add no-std::no-alloc category
dcbc3e0 Release 1.0.165
0289d31 Fix -Zminimal-versions build
015e397 No need for single-element vec for chaining one element
6a9a21f Resolve useless_conversion clippy lint in test
81ac54b Resolve redundant_closure_call clippy lint
6b4e755 Resolve explicit_iter_loop pedantic clippy lint
b053b4f Touch up early return in Enum checks
c0ba323 Support a manual trigger on CI workflow
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump async-trait from 0.1.68 to 0.1.69 (#147)
- Bumps async-trait from 0.1.68 to 0.1.69.
- Release notes
Sourced from async-trait's releases.
- 0.1.69
- Resolve new diverging_sub_expression clippy lint in generated code
- 
Commits
- 78640d2 Release 0.1.69
b59322e Suppress diverging_sub_expression clippy lint in generated code
fbd310e Resolve explicit_iter_loop pedantic clippy lint
603c57a Remove .clippy.toml in favor of respecting rust-version from Cargo.toml
454b0bd Show error details during miri setup in CI
486fe7e Ignore non_minimal_cfg clippy lint in test
49cdc5f Update ui test suite to nightly-2023-05-14
3983ba5 Revert "Temporarily disable miri CI"
8c0ed45 Temporarily disable miri CI
c0f1e53 Update ui test suite to nightly-2023-05-02
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump num_cpus from 1.13.1 to 1.16.0 (#146)
- Bumps num_cpus from 1.13.1 to 1.16.0.
- Release notes
Sourced from num_cpus's releases.
- v1.16.0
Features
- add support for AIX operating system
- Fixes
- update hermit-abi to 0.3.0
- New Contributors
- @​mkroening made their first contribution in seanmonstar/num_cpus#128
@​ecnelises made their first contribution in seanmonstar/num_cpus#123
- v1.15.0
Fixes
- update hermit-abi
- New Contributors
- @​striezel made their first contribution in seanmonstar/num_cpus#126
@​buffet made their first contribution in seanmonstar/num_cpus#127
- v1.14.0
Features
- Support cgroups v2 by @​OrionNebula in seanmonstar/num_cpus#125
Do not attempt to open files in Miri by @​RalfJung in seanmonstar/num_cpus#121
- New Contributors
- @​RalfJung made their first contribution in seanmonstar/num_cpus#121
@​OrionNebula made their first contribution in seanmonstar/num_cpus#125
- 
Changelog
Sourced from num_cpus's changelog.
- v1.16.0
Features
- add support for AIX operating system
- Fixes
- update hermit-abi to 0.3.0
- v1.15.0
Fixes
- update hermit-abi
- v1.14.0
Features
- add support for cgroups v2
Skip reading files in Miri
- 
Commits
- 7c03fc9 v1.16.0
cd76834 add funding file
f3e7081 Support AIX operating system (#123)
edf035c Update CI badges (#132)
3b6e5f0 Bump hermit-abi to 0.3.0 and don't restrict hermit-abi architectures (#128)
5bea3c7 v1.15.0
ccb5a67 chore: update hermit-abi to 0.2.6 (#127)
e437b9d ci: update actions/checkout in GitHub Actions workflows to v3 (#126)
90373f3 v1.14.0
4f0f2a4 Support reading cpu.max from cgroups v2 (#125)
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde_yaml from 0.9.21 to 0.9.22 (#144)
- ⚠️  Dependabot is rebasing this PR ⚠️
Rebasing might not happen immediately, so don't worry if this takes some time.
Note: if you make any changes to this PR yourself, they will take precedence over the rebase.
- Bumps serde_yaml from 0.9.21 to 0.9.22.
- Release notes
Sourced from serde_yaml's releases.
- 0.9.22
- Update indexmap dependency to version 2
- 
Commits
- 060eb86 Release 0.9.22
b12ad38 Merge pull request #377 from dtolnay/indexmap
c418ad5 Update indexmap dependency to version 2
f1cd9e6 Remove .clippy.toml in favor of respecting rust-version from Cargo.toml
147103c Show error details during miri setup in CI
622553f Fix unused import warnings in test under cfg miri
2037c7e Fix new unused_mut detected by nightly-2023-04-30
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump uuid from 1.3.4 to 1.4.0 (#145)
- Bumps uuid from 1.3.4 to 1.4.0.
- Release notes
Sourced from uuid's releases.
- 1.4.0
What's Changed
- Fixed wasm tests not running due to incorrect attribute target by @​kmusick in uuid-rs/uuid#688
Fixing issue with Cloudflare Workers and wasm32-unknown-unknown when using now() by @​kmusick in uuid-rs/uuid#690
Add borsh support by @​grovesNL in uuid-rs/uuid#686
Fix some timestamp generation by @​KodrAus in uuid-rs/uuid#691
Prepare for 1.4.0 release by @​KodrAus in uuid-rs/uuid#692
- New Contributors
- @​kmusick made their first contribution in uuid-rs/uuid#688
@​grovesNL made their first contribution in uuid-rs/uuid#686
- Full Changelog: uuid-rs/uuid@1.3.4...1.4.0
- Commits
- 0fc3101 Merge pull request #692 from uuid-rs/cargo/1.4.0
d9f72db prepare for 1.4.0 release
cb2aac0 Merge pull request #691 from uuid-rs/fix/timestamp-gen
0cb9232 add missing wasm import
cb80ba2 run fmt
8babf97 add missing wasm test attr
759c971 fix a warning in arbitrary support
646bd98 wrap rather than overflow timestamps
7da3f69 remove dbg call from wasm-based timestamp
952f75f Merge pull request #686 from pod2co/borsh
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde_json from 1.0.97 to 1.0.99 (#143)
- Bumps serde_json from 1.0.97 to 1.0.99.
- Release notes
Sourced from serde_json's releases.
- v1.0.99
- Support serializing serde's option type in a map key (#1030, thanks @​LPGhatguy)
- v1.0.98
- Update indexmap dependency used by "preserve_order" feature to version 2
- 
Commits
- b4ec50c Release 1.0.99
1153052 Merge pull request 1030 from SecondHalfGames/map-key-serialize-some
ba29a89 Release 1.0.98
9508e50 Merge pull request #1031 from serde-rs/indexmap
706fc2b Do all CI builds with old rustc using shim crate
d4c98d0 Move serde_json_test crate to own workspace
e09d78f Update indexmap dependency used for preserve_order feature to version 2
5145907 Delete unneeded conditional on preserve_order steps in CI
b0fa978 Change MapKeySerializer::serialize_some to fall through instead of erroring
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump itertools from 0.10.5 to 0.11.0 (#142)
- Bumps itertools from 0.10.5 to 0.11.0.
- Changelog
Sourced from itertools's changelog.
- 0.11.0
Breaking
- Make Itertools::merge_join_by also accept functions returning bool (#704)
Implement PeekingNext transitively over mutable references (#643)
Change with_position to yield (Position, Item) instead of Position<Item> (#699)
- Added
- Add Itertools::take_while_inclusive (#616)
Implement PeekingNext for PeekingTakeWhile (#644)
Add EitherOrBoth::{just_left, just_right, into_left, into_right, as_deref, as_deref_mut, left_or_insert, right_or_insert, left_or_insert_with, right_or_insert_with, insert_left, insert_right, insert_both} (#629)
Implement Clone for CircularTupleWindows (#686)
Implement Clone for Chunks (#683)
Add Itertools::process_results (#680)
- Changed
- Use Cell instead of RefCell in Format and FormatWith (#608)
CI tweaks (#674, #675)
Document and test the difference between stable and unstable sorts (#653)
Fix documentation error on Itertools::max_set_by_key (#692)
Move MSRV metadata to Cargo.toml (#672)
Implement equal with Iterator::eq (#591)
- 
Commits
- 62a6401 chore: Release itertools version 0.11.0
8f17227 remove no-dev-version Cargo release directive
bb2fc59 Merge #706
0ef6b7e prepare v0.11.0 release
c5b64c9 Merge #616
eb561e5 Merge #704
3e92550 Merge #643
8184e4c Merge #644
ae31559 MergeJoinBy also accept functions returning bool
8bc377e Merge #629
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde_json from 1.0.96 to 1.0.97 (#141)
- Bumps serde_json from 1.0.96 to 1.0.97.
- Release notes
Sourced from serde_json's releases.
- v1.0.97
- Add io_error_kind() method to serde_json::Error: fn io_error_kind(&self) -> Option<std::io::ErrorKind> (#1026)
- 
Commits
- a0ddb25 Release 1.0.97
8b681ff Merge pull request #1026 from dtolnay/errorkind
9308d97 Add Error::io_error_kind
136b773 Merge pull request #1025 from dtolnay/io
207a57b Standardize on "I/O" instead of "IO"
6fda385 Merge pull request 1021 from ndmitchell/patch-2
009a53b Switch to using null
931ee23 Show error details during miri setup in CI
0d7b0d3 Add an example of producing a Null in a json! literal
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump uuid from 1.3.3 to 1.3.4 (#140)
- Bumps uuid from 1.3.3 to 1.3.4.
- Release notes
Sourced from uuid's releases.
- 1.3.4
What's Changed
- Add wasm32-wasi support with tests by @​acfoltzer in uuid-rs/uuid#677
Fix up MSRV build in CI by @​KodrAus in uuid-rs/uuid#679
fix: keep the order when filling random bytes by @​Hanaasagi in uuid-rs/uuid#682
Prepare for 1.3.4 release by @​KodrAus in uuid-rs/uuid#683
- New Contributors
- @​acfoltzer made their first contribution in uuid-rs/uuid#677
@​Hanaasagi made their first contribution in uuid-rs/uuid#682
- Full Changelog: uuid-rs/uuid@1.3.3...1.3.4
- Commits
- 07052be Merge pull request #683 from uuid-rs/cargo/1.3.4
80ec18c prepare for 1.3.4 release
4297536 Merge pull request #682 from Hanaasagi/fix-index
3af4733 fix: keep the order when filling random bytes
6188ecf Merge pull request #679 from uuid-rs/ci/msrv-build
e582a3d Just check v4 for MSRV
b466522 fix up MSRV build in CI
a0d6eb6 Merge pull request #677 from acfoltzer/wasm32-wasi
403f845 add installed wasmtime to GITHUB_PATH
f74e05e add wasm32-wasi target in CI
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump liquid from 0.26.3 to 0.26.4 (#139)
- Bumps liquid from 0.26.3 to 0.26.4.
- Changelog
Sourced from liquid's changelog.
- [0.26.4] - 2023-06-09
Features
- Initial support for render tag
- 
Commits
- 0c2088f chore: Release
6746ae9 docs: Update changelog
a470b62 Merge pull request #506 from epage/render
3f46fa4 docs: Fix typos
c248b49 feat: Add render tag
680b5f3 feat: Add SandboxedStackFrame
f6bf212 refactor: Expose for_loop helpers
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump serde from 1.0.163 to 1.0.164 (#138)
- Bumps serde from 1.0.163 to 1.0.164.
- Release notes
Sourced from serde's releases.
- v1.0.164
- Allowed enum variants to be individually marked as untagged (#2403, thanks @​dewert99)
- 
Commits
- 107018c Release 1.0.164
a398237 Point out serde(untagged) variants which are out of order
b63c65d Merge pull request #2470 from dtolnay/contentref
f60324e Reuse a single ContentRefDeserializer throughout untagged enum deserialization
361c23a Simplify enumerate().find(...) -> Iterator::position
43b23c7 Format PR 2403 with rustfmt
6081497 Resolve semicolon_if_nothing_returned pedantic clippy lint
48e5753 Allowed Enum variants to be individually marked as untagged (#2403)
bbba632 Revert "Ui tests with compile_error resolved at call site"
e77db40 Ui tests with compile_error resolved at call site
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump url from 2.3.1 to 2.4.0 (#137)
- Bumps url from 2.3.1 to 2.4.0.
- Commits
- a3e07c7 Merge pull request #840 from servo/update-ver
1317d9d update dependencies
a25f3a8 Update url to v2.4.0
2a12745 Update idna to 0.4.0
1e6fd5d Update form_urlencoded to 1.2.0
90833ff Update percent-encoding to 2.3.0
f5b961c Update data-url to 0.3.0
0e25146 Merge pull request #839 from servo/fix-838
21f32d6 Fix lint
df88a29 Also fix issue where path segment could be confused with drive letter because...
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump liquid from 0.26.2 to 0.26.3 (#136)
- Bumps liquid from 0.26.2 to 0.26.3.
- Changelog
Sourced from liquid's changelog.
- [0.26.3] - 2023-06-02
Internal
- Update dependencies
- 
Commits
- 35314b6 chore: Release
38fb7a6 docs: Update changelog
868322f Merge pull request #505 from epage/syn
7bc7874 Merge pull request #504 from epage/update
50bb072 Merge pull request #501 from cobalt-org/renovate/compatible-(dev)
7405787 chore(deps): update compatible (dev)
38b296e Merge pull request #502 from cobalt-org/renovate/criterion-0.x
7139f50 chore(derive): Update to syn v2
73adafb chore: Upgrade thiserror to get syn v2
54e7505 chore: Upgrade serde to get syn v2
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump figment from 0.10.9 to 0.10.10 (#135)
- Bumps figment from 0.10.9 to 0.10.10.
- Commits
- fdfbf58 New version: 0.10.10.
051136f Improve conflict resolution docs.
174d235 Add 'Figment::admerge()'.
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump csv from 1.2.1 to 1.2.2 (#134)
- Bumps csv from 1.2.1 to 1.2.2.
- Commits
- 574ae1f 1.2.2
0cf3f6c api: add Writer::get_ref
d24eac1 doc: make setup version independent
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump liquid from 0.26.1 to 0.26.2 (#133)
- Bumps liquid from 0.26.1 to 0.26.2.
- Changelog
Sourced from liquid's changelog.
- [0.26.2] - 2023-05-28
Compatibility
Bump MSRV to 1.65
Fix
- Serialization leaves off subseconds when not needed
Deserialiation supports with and without subseconds
- 
Commits
- 6860b51 chore: Release
ec19a47 docs: Update changelog
b601273 Merge pull request #500 from tglman/master
ad3504d fix: Correct date time serde formats
71f80ff Merge pull request #498 from epage/template
5bf6434 style: Make clippy happy
e7f37e4 chore: Update from '_rust/main' template
4d44cd7 chore: Update precommit hooks
2b6bb28 chore(ci): Catch clippy config failures
80d4cdd chore: Remove clippy lint past MSRV (needs 1.67)
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump figment from 0.10.8 to 0.10.9 (#132)
- Bumps figment from 0.10.8 to 0.10.9.
- Commits
- 334cf03 New version: 0.10.9.
ab4c81b Add 'Figment::adjoin()' to concat vectors.
a003ffa Update 'clap' (dev-dependency) to 4.
b0971cd Update 'toml' to 0.7.
0118606 Use PCRE git grep to work around macOS git bug.
7e37b78 Document key paths in 'Serialize' more clearly.
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump console from 0.15.6 to 0.15.7 (#131)
- Bumps console from 0.15.6 to 0.15.7.
- Changelog
Sourced from console's changelog.
- 0.15.7
Enhancements
- Set an appropriate lower version of libc for macos changes.
Improved behavior of read_single_key so it does not disturb other
threads quite as much. (#165)
More reliably reset raw mode in terminal. (#171)
- 
Commits
- 842b376 0.15.7
03aefb9 Mention changelog entry
85c4de4 More reliably reset raw mode (#171)
55e4762 Update changelog entries
bda931c Don't touch output config attributes in read_single_key() (#165)
b5c80cc Raise libc minimums to 0.2.99
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: Bump console from 0.15.5 to 0.15.6 (#130)
- Bumps console from 0.15.5 to 0.15.6.
- Changelog
Sourced from console's changelog.
- 0.15.6
Enhancements
- Switch to select() on macOS for polling on TTYs to work around
a macOS bug. (#169)
Added blink fast and strikethrough attributes. (#159)
- 
Commits
- d71320f 0.15.6
bf43c20 Added missing changelog entry
10bb108 Added changelog entry
66cb2aa Prefer select() over poll() on macos for ttys (#169)
ab9cd9d Added blink fast and strikethrough attributes (#159)
7c7a7d9 Fix CI (#160)
572e9a5 Update windows-sys requirement from 0.42.0 to 0.45.0 (#157)
33f2754 Added Hash derive to Key (#156)
560342b Configure WASM tests an try isatty on wasi (#155)
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump reqwest from 0.11.17 to 0.11.18 (#129)
- Bumps reqwest from 0.11.17 to 0.11.18.
- Release notes
Sourced from reqwest's releases.
- v0.11.18
What's Changed
- Fix RequestBuilder::json() method from overriding a previously set content-type header. An existing value will be left in place.
Upgrade internal dependencies for rustls and compression.
- New Contributors
- @​flyingalex made their first contribution in seanmonstar/reqwest#1833
@​cpu made their first contribution in seanmonstar/reqwest#1791
- 
Changelog
Sourced from reqwest's changelog.
- v0.11.18
- Fix RequestBuilder::json() method from overriding a previously set content-type header. An existing value will be left in place.
Upgrade internal dependencies for rustls and compression.
- 
Commits
- 00be85e v0.11.18
a0b5ea5 deps: update rustls v0.20.1 -> v0.21.0 (#1791)
b13ca4b bug: fix custom content-type overidden by json method (#1833)
eca2a2f CI: Enable dependabot for GitHub Action Workflow (#1831)
9de702c Speedup CI (#1830)
7e7b116 deps: Update async-compression v0.3.13 => v0.4.0 (#1828)
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump uuid from 1.3.2 to 1.3.3 (#128)
- Bumps uuid from 1.3.2 to 1.3.3.
- Release notes
Sourced from uuid's releases.
- 1.3.3
What's Changed
- Use sha hash for checkout action and remove others by @​KodrAus in uuid-rs/uuid#671
Hard deprecate Timestamp::to_unix_nanos by @​KodrAus in uuid-rs/uuid#673
Prepare for 1.3.3 release by @​KodrAus in uuid-rs/uuid#674
- Full Changelog: uuid-rs/uuid@1.3.2...1.3.3
- Commits
- 2cfe9f1 Merge pull request #674 from uuid-rs/cargo/1.3.3
2bbbb74 prepare for 1.3.3 release
534691e Merge pull request #673 from uuid-rs/chore/hard-deprecation
147d4b3 update build agents
d89c69d hard deprecate Timestamp::to_unix_nanos
6d22d0c Merge pull request #671 from uuid-rs/ci/cleanups
0536908 use sha hash for checkout action and remove others
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump serde from 1.0.162 to 1.0.163 (#127)
- Bumps serde from 1.0.162 to 1.0.163.
- Release notes
Sourced from serde's releases.
- v1.0.163
- Eliminate build script from serde_derive crate to slightly reduce build time (#2442, thanks @​taiki-e)
- 
Commits
- fccb949 Release 1.0.163
a139ab2 Adjust PR 2446 with less overgeneralized name
1d910a4 Format with rustfmt 1.5.2-nightly
ee9166e Revise comments on the FlatMapDeserializer entry taker
b5a9eff Resolve while_let_on_iterator clippy lint
9441a29 Merge pull request #2446 from Mingun/dedup2
ab6588e Extract duplicated code into a function
1d11f03 Extract logic of taking flattened fields into a function
e11d01f Remove constructors for FlatMapAccess and FlatStructAccess
a901f50 FlatMapAccess and FlatStructAccess does not need to be public
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump serde from 1.0.160 to 1.0.162 (#126)
- Bumps serde from 1.0.160 to 1.0.162.
- Release notes
Sourced from serde's releases.
- 1.0.162
- 
Support deserializing flattened adjacently tagged enums from data formats which represent fields as bytes, such as the csv crate (#2377, thanks @​mfro)
#[derive(Deserialize)]
pub struct Record {
    common: u64,
    #[serde(flatten)]
    kind: Kind,
}
#[derive(Deserialize)]
#[serde(tag = "kind", content = "parameter", rename_all = "lowercase")]
enum Kind {
Foo(u64),
Bar(bool),
}
common,kind,parameter
1,foo,42
2,bar,true
- v1.0.161
- Improve error messages produced by serde_test on test failure (#2435, thanks @​Mingun)
- 
Commits
- 99f165b Release 1.0.162
2fb5560 Attempt to generate just one copy of TagContentOtherFieldVisitor's field matc...
bd653ab Format PR 2377 with rustfmt
b5d68ae Merge pull request #2377 from mfro/master
624879c Merge pull request #2441 from dtolnay/test
bd9e9ab Reimplement tests that touched serde_test internal API
3e4a23c Release 1.0.161
6326cee Don't panic in serde_test on running out of tokens
8f4d37c Convert serde_test's assert_next_token from macro to function
1b8290b Convert serde_test's unexpected from macro to function
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump uuid from 1.3.1 to 1.3.2 (#125)
- Bumps uuid from 1.3.1 to 1.3.2.
- Release notes
Sourced from uuid's releases.
- 1.3.2
What's Changed
- Create SECURITY.md by @​KodrAus in uuid-rs/uuid#668
Faster as_u128 and to_u128_le by @​pkoenig10 in uuid-rs/uuid#669
prepare for 1.3.2 release by @​KodrAus in uuid-rs/uuid#670
- New Contributors
- @​pkoenig10 made their first contribution in uuid-rs/uuid#669
- Full Changelog: uuid-rs/uuid@1.3.1...1.3.2
- Commits
- 52867d4 Merge pull request #670 from uuid-rs/cargo/1.3.2
f0c3f1a prepare for 1.3.2 release
a78ee0a Merge pull request #669 from pkoenig10/as_u128
7978c5a Faster as_u128 and to_u128_le
f725e58 Merge pull request #668 from uuid-rs/KodrAus-patch-1
f1919c1 Create SECURITY.md
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump reqwest from 0.11.16 to 0.11.17 (#124)
- Bumps reqwest from 0.11.16 to 0.11.17.
- Release notes
Sourced from reqwest's releases.
- v0.11.17
What's Changed
- Upgrade internal dependencies of Experimental HTTP/3 to use quinn v0.9
(wasm) Fix blob url support
- New Contributors
- @​skyf0l made their first contribution in seanmonstar/reqwest#1797
- 
Changelog
Sourced from reqwest's changelog.
- v0.11.17
- Upgrade internal dependencies of Experimental HTTP/3 to use quinn v0.9
(wasm) Fix blob url support
- 
Commits
- eeca649 v0.11.17
b4d5ab0 update h3 and h3-quinn to 0.0.2 (#1811)
2fa69ad wasm: blob url support (#1797)
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump serde_json from 1.0.95 to 1.0.96 (#123)
- Bumps serde_json from 1.0.95 to 1.0.96.
- Release notes
Sourced from serde_json's releases.
- v1.0.96
- Guarantee that to_writer only writes valid UTF-8 strings (#1011, thanks @​stepancheg)
- 
Commits
- 187f7da Release 1.0.96
41199cc Merge pull request #1011 from stepancheg/utf-8
cd5ed82 Document to_writer only writes valid UTF-8 strings
ce53b86 Fix needless_borrow clippy lint in test
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump serde from 1.0.159 to 1.0.160 (#122)
- Bumps serde from 1.0.159 to 1.0.160.
- Release notes
Sourced from serde's releases.
- v1.0.160
- Make derived serializer/deserializer internals doc(hidden) (#2426, thanks @​compiler-errors)
- 
Commits
- 0c6a2bb Release 1.0.160
a80d830 Merge pull request #2426 from compiler-errors/dont-doc-private
5f3fd99 Make serializer/deserializer internals doc(hidden)
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump uuid from 1.3.0 to 1.3.1 (#121)
- Bumps uuid from 1.3.0 to 1.3.1.
- Release notes
Sourced from uuid's releases.
- 1.3.1
What's Changed
- Update syn requirement from 1.0.80 to 2.0.5 by @​dependabot in uuid-rs/uuid#663
Update windows-sys requirement from 0.45.0 to 0.48.0 by @​dependabot in uuid-rs/uuid#665
remove some extra chars in a comment by @​KodrAus in uuid-rs/uuid#666
Prepare for 1.3.1 release by @​KodrAus in uuid-rs/uuid#667
- Full Changelog: uuid-rs/uuid@1.3.0...1.3.1
- Commits
- 87082b9 Merge pull request #667 from uuid-rs/cargo/1.3.1
a367481 prepare for 1.3.1 release
ee19f3d Merge pull request #666 from uuid-rs/KodrAus-patch-2
8ccdf67 remove some extra chars in a comment
b111b12 Merge pull request #665 from uuid-rs/dependabot/cargo/windows-sys-0.48.0
faac14c Update windows-sys requirement from 0.45.0 to 0.48.0
efa4686 Merge pull request #663 from uuid-rs/dependabot/cargo/syn-2.0.5
1795337 Update syn requirement from 1.0.80 to 2.0.5
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump serde_yaml from 0.9.19 to 0.9.21 (#120)
- Bumps serde_yaml from 0.9.19 to 0.9.21.
- Release notes
Sourced from serde_yaml's releases.
- 0.9.21
- Make Tag::new panic if given empty string, since YAML has no syntax for an empty tag
- 0.9.20
- Allow an empty YAML document to deserialize to None or Value::Null, in addition to the previously supported empty vector, empty map, and struct with no required fields
- 
Commits
- 8057fad Release 0.9.21
19a7bd3 Merge pull request #371 from dtolnay/emptytag
ebb4b7a Fix deserialization of tag !<%21>
879a57f Factor out conversion function from libyaml Tag to tag string
e3b9a02 Merge pull request #370 from dtolnay/emptytag
ef43549 Treat Tag::new("!") as tag %21, instead of empty tag
221154c Merge pull request #369 from dtolnay/emptytag
39a866c Reuse Tag visitor in Value's Deserialize impl
80e53a5 Merge pull request #368 from dtolnay/emptytag
f668f71 Preserve location information better when parsing invalid tag
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump futures from 0.3.26 to 0.3.28 (#119)
- Bumps futures from 0.3.26 to 0.3.28.
- Release notes
Sourced from futures's releases.
- 0.3.28
- Update to syn 2. This raises MSRV of utility crates to 1.56. (#2730, #2733)
Fix bug in FlattenUnordered (#2726, #2728)
- 0.3.27
- Add TryFlattenUnordered (#2577, #2590, #2606, #2607)
Add AbortHandle::is_aborted (#2710)
Add AbortRegistration::handle (#2712)
Make BiLock strict-provenance compatible (#2716)
- 
Changelog
Sourced from futures's changelog.
- 0.3.28 - 2023-03-30
- Update to syn 2. This raises MSRV of utility crates to 1.56. (#2730, #2733)
Fix bug in FlattenUnordered (#2726, #2728)
- 0.3.27 - 2023-03-11
- Add TryFlattenUnordered (#2577, #2590, #2606, #2607)
Add AbortHandle::is_aborted (#2710)
Add AbortRegistration::handle (#2712)
Make BiLock strict-provenance compatible (#2716)
- 
Commits
- 1685f8b Release 0.3.28
206b12b Update to syn 2 (#2730)
98e80d4 Bump MSRV of utility crates to 1.56 (#2733)
a3f80e6 Fix unknown_lints and unused_imports warnings in test (#2732)
3bee396 SelectAll doesn't need pin-project (#2729)
6a436eb Don't ignore empty state polling (#2728)
94e020d Use Waker::will_wake() to avoid a cloning op (#2723)
a730a19 FlattenUnordered: always replace inner wakers (#2726)
890f893 Fix rustdoc warning
4b86e46 Use setup-cross-toolchain-action instead of cross
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump serde_json from 1.0.94 to 1.0.95 (#117)
- ⚠️  Dependabot is rebasing this PR ⚠️
Rebasing might not happen immediately, so don't worry if this takes some time.
Note: if you make any changes to this PR yourself, they will take precedence over the rebase.
- Bumps serde_json from 1.0.94 to 1.0.95.
- Release notes
Sourced from serde_json's releases.
- v1.0.95
- Preserve f32 precision when serializing f32 -> serde_json::Value -> JSON string in "arbitrary_precision" mode (#1004, #1005)
- 
Commits
- 4ea38c4 Release 1.0.95
731886c Merge pull request #1005 from dtolnay/f32cast
c9bff92 Fix PartialEq between Value and f32
06f3443 Eliminate f32-to-f64 casting in arbitrary_precision mode
b0990a5 Add regression test for issue 1004
02e5833 Update fuzz crate gitignore to ignore coverage dir
4b96996 No longer test so many old compiler versions
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump reqwest from 0.11.15 to 0.11.16 (#118)
- ⚠️  Dependabot is rebasing this PR ⚠️
Rebasing might not happen immediately, so don't worry if this takes some time.
Note: if you make any changes to this PR yourself, they will take precedence over the rebase.
- Bumps reqwest from 0.11.15 to 0.11.16.
- Release notes
Sourced from reqwest's releases.
- v0.11.16
What's Changed
- Fix building docs on docs.rs by @​NobodyXu in seanmonstar/reqwest#1789
Set 'rust-version` in Cargo metadata and use it in the MSRV build job by @​nickelc in seanmonstar/reqwest#1793
- 
Changelog
Sourced from reqwest's changelog.
- v0.11.16
- Chore: set MSRV in Cargo.toml.
Docs: fix build on docs.rs
- 
Commits
- 7047669 v0.11.16
cc47ef1 Set 'rust-version` in Cargo metadata and use it in the MSRV build job (#1793)
7fdd014 docs: Fix building on docs.rs (#1789)
bf7ff55 chore: update changelog for 0.11.15
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump serde from 1.0.158 to 1.0.159 (#116)
- Bumps serde from 1.0.158 to 1.0.159.
- Release notes
Sourced from serde's releases.
- v1.0.159
- Accept empty #[serde()] attribute (#2422)
- 
Commits
- d6de911 Release 1.0.159
04af322 Merge pull request #2422 from dtolnay/emptyattr
4cb8d07 Accept empty #[serde()] attribute
6ab55a1 Add regression test for issue 2415
acfd19c Release serde_derive_internals 0.27.0
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump async-trait from 0.1.67 to 0.1.68 (#115)
- Bumps async-trait from 0.1.67 to 0.1.68.
- Release notes
Sourced from async-trait's releases.
- 0.1.68
- Improve error message if an async fn is written without a function body in an impl block
- 
Commits
- dce6060 Release 0.1.68
346f050 Merge pull request #244 from dtolnay/verbatimfn
344a4f2 Handle async impl fn without body
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump rocket from 0.5.0-rc.2 to 0.5.0-rc.3 (#114)
- Bumps rocket from 0.5.0-rc.2 to 0.5.0-rc.3.
- Release notes
Sourced from rocket's releases.
- Rocket v0.5.0-rc.3
See the CHANGELOG, news article, migration guide, and FAQ for more information.
- Changelog
Sourced from rocket's changelog.
- Version 0.5.0-rc.3 (Mar 23, 2023)
Major Features and Improvements
- 
Added a [max_blocking] configuration parameter.
The parameter sets a limit on the number of threads used by blocking tasks.
- 
Added an [ip_header] "real IP" header configuration parameter.
The parameter allows modifying the header that Rocket attempts to use to retrieve the "real IP"
address of the client via Request methods like [Request::client_ip()]. Additionally, the
change allows disabling the use of any such header entirely.
- 
A [pool()] method is emitted by [rocket_sync_db_pools] for code-generated pools.
The method returns an opaque reference to a type that can be used to retrieve pooled connections
outside of a request handling context.
- 
Raw binary form field data can be retrieved using the &[u8] form guard.
- 
Data guards are now eligible [sentinels].
- 
General Improvements
- Final launch messages are now always logged, irrespective of profile.
Only functions that return Rocket<Build> are now #[must_use], not all Rocket<P>.
Fixed mismatched form field names in errors under certain conditions in [FromForm] derive.
The [FromForm] derive now collects all errors that occur.
Data pools are now gracefully shutdown in [rocket_sync_db_pools].
Added [Metadata::render()] in [rocket_dyn_templates] for direct template rendering.
Rocket salvages more information from malformed requests for error catchers.
The cookie secure feature is now properly conditionally enabled.
Data before encapsulation boundaries in TLS keys is allowed and ignored.
Support for TLS keys in SEC1 format was added.
Rocket now warns when a known secret key is configured.
A panic that could occur on shutdown in rocket_sync_db_pools was fixed.
- Known Media Types
- Added MP3: audio/mpeg.
Added CBZ: application/vnd.comicbook+zip, extension .cbz.
Added CBR: application/vnd.comicbook-rar, extension .cbr.
Added RAR: application/vnd.rar, extension .rar.
Added EPUB: application/epub+zip, extension .epub.
Added OPF: application/oebps-package+xml, extension .opf.
Added XHTML: application/xhtml+xml, extension .xhtml.
- Trait Implementations
- 
... (truncated)
- 
Commits
- 91f6288 New version: 0.5.0-rc.3.
f800d52 Update CHANGELOG, add news article for 0.5.0-rc.3.
cf1748d Add CHANGELOG for 0.5.0-rc.3.
14b8a08 Fix doc re-export inlines in crate root.
28569e8 Improve section headings in README.md.
89919aa Fix several typos.
0e6b786 Fix typo in docs.
e86d503 Fix typo in updating guide: 'phase' -> 'faze'.
380e75c Fix minor typo.
6742041 Remove unnecessary word in state docs.
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump serde from 1.0.157 to 1.0.158 (#113)
- ⚠️  Dependabot is rebasing this PR ⚠️
Rebasing might not happen immediately, so don't worry if this takes some time.
Note: if you make any changes to this PR yourself, they will take precedence over the rebase.
- Bumps serde from 1.0.157 to 1.0.158.
- Release notes
Sourced from serde's releases.
- v1.0.158
- Fix "expected serde crate attribute to be a string" error when using macro_rules metavariable inside of serde attribute: #[serde(crate = $serde_path)] (#2409)
- 
Commits
- e305810 Release 1.0.158
dc200a6 Reformat comments of non-public serde_derive internals
2c0999a Merge pull request #2410 from serde-rs/attrvalue
dd460f8 Check for None-delimited group in attribute value
c3d637f Add regression test for issue 2409
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump thiserror from 1.0.39 to 1.0.40 (#110)
- ⚠️  Dependabot is rebasing this PR ⚠️
Rebasing might not happen immediately, so don't worry if this takes some time.
Note: if you make any changes to this PR yourself, they will take precedence over the rebase.
- Bumps thiserror from 1.0.39 to 1.0.40.
- Release notes
Sourced from thiserror's releases.
- 1.0.40
- Update syn dependency to 2.x
- 
Commits
- 3cec8c4 Release 1.0.40
2c65cea Merge pull request #227 from dtolnay/syn
fb8b81f Update to syn 2
0e45dde Merge pull request #226 from dtolnay/tokenspan
490dc01 Eliminate unneeded use of Spanned trait on single tokens
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump async-trait from 0.1.57 to 0.1.67 (#111)
- ⚠️  Dependabot is rebasing this PR ⚠️
Rebasing might not happen immediately, so don't worry if this takes some time.
Note: if you make any changes to this PR yourself, they will take precedence over the rebase.
- Bumps async-trait from 0.1.57 to 0.1.67.
- Release notes
Sourced from async-trait's releases.
- 0.1.67
- Update syn dependency to 2.x
- 0.1.66
- Set html_root_url attribute
- 0.1.65
- Fix interaction with rustc's single_use_lifetimes lint (#238, #239)
- 0.1.64
- Suppress async_yields_async clippy correctness lint in generated code (#236, #237)
- 0.1.63
- Do not require Sync on unused shared reference arguments (#232, #233)
Make expansion of nested _ and .. patterns edition independent (#234, #235)
- 0.1.62
- Improve error message involving elided lifetimes (#229)
- 0.1.61
- Fix async function signatures that involve #[cfg(...)] attributes on parameters (#227, thanks @​azriel91)
- 0.1.60
- Documentation improvements
- 0.1.59
- Support self: Arc<Self> async methods that have a default implementation provided by the trait (#210)
- 0.1.58
- Improve rust-analyzer "go to definition" on the method names of an async trait (#218)
- 
Commits
- f8a8650 Release 0.1.67
d7a9cae Merge pull request #241 from dtolnay/syn
15fd282 Ignore match_like_matches_macro clippy lint
032c150 Update to syn 2
0d0a346 Update ui test suite to nightly-2023-03-17
9a9b322 Release 0.1.66
4cd54b8 Set html_root_url
ccb55b8 Release 0.1.65
7eea88b Merge pull request #239 from dtolnay/singleuse
5883ac8 Delete replacement of elided lifetimes in impl heading
Additional commits viewable in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump reqwest from 0.11.14 to 0.11.15 (#112)
- Bumps reqwest from 0.11.14 to 0.11.15.
- Release notes
Sourced from reqwest's releases.
- v0.11.15
What's Changed
- Add RequestBuilder methods to split and reconstruct from its parts.
Add experimental HTTP/3 support. 🧪3️🎉
Fix connection_verbose to log write_vectored calls.
(wasm) Make requests actually cancel if the future is dropped.
- New Contributors
- @​jneem made their first contribution in seanmonstar/reqwest#1755
@​TurnOfACard made their first contribution in seanmonstar/reqwest#1762
@​j7nw4r made their first contribution in seanmonstar/reqwest#1765
- Full Changelog: seanmonstar/reqwest@v0.11.14...v0.11.15
- Changelog
Sourced from reqwest's changelog.
- v0.11.15
- Add RequestBuilder methods to split and reconstruct from its parts.
Add experimental HTTP/3 support.
Fix connection_verbose to log write_vectored calls.
(wasm) Make requests actually cancel if the future is dropped.
- 
Commits
- 56190bd v0.11.15
df2b3ba wasm: fix premature abort for streaming bodies (#1782)
4db868b Make HTTP/3 feature more unstable (#1780)
57a8a01 Add Experimental HTTP/3 Support (#1599)
119366e async/request: add methods to split and reassemble a RequestBuilder (#1770)
673449a docs: fix wording on main docs page (#1765)
c2ac875 Clarify the documentation to show that get will fail if the total download ti...
bb8fec4 Make wasm requests cancel when the future drops. (#1755)
bdd4db0 Fix connection_verbose to log write_vectored calls (#1737)
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore: bump serde from 1.0.156 to 1.0.157 (#109)
- Bumps serde from 1.0.156 to 1.0.157.
- Release notes
Sourced from serde's releases.
- v1.0.157
- Update syn dependency to 2.x
- 
Commits
- 479a00a Release 1.0.157
c42e7c8 Reflect serde_derive required compiler in build script and rust-version metadata
5b8e065 Ignore single_match_else pedantic clippy lint in serde_derive_internals
9fc0d13 Merge pull request #2406 from dtolnay/nestedmeta
bc22641 Rewrite attribute parser using parse_nested_meta
0509810 Update compiler version for serde_derive in readme
5b23634 Merge pull request #2405 from dtolnay/syn
32f0d00 Update to syn 2
9d87851 Merge pull request #2404 from dtolnay/attributeexpr
c0296ee Add ui test of malformed attribute containing expression
See full diff in compare view
- 
Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting @dependabot rebase.
- 
Dependabot commands and options
- You can trigger Dependabot actions by commenting on this PR:
- @dependabot rebase will rebase this PR
@dependabot recreate will recreate this PR, overwriting any edits that have been made to it
@dependabot merge will merge this PR after your CI passes on it
@dependabot squash and merge will squash and merge this PR after your CI passes on it
@dependabot cancel merge will cancel a previously requested merge and block automerging
@dependabot reopen will reopen this PR if it is closed
@dependabot close will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
@dependabot ignore this major version will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this minor version will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
@dependabot ignore this dependency will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- chore(ci): fixed bad chomp
- chore(ci): skip publish steps when cz doesn't bump

## 0.2.11 (2023-03-17)


- fix(ci): kodiak config to fix dependabot PRs
- chore: bump num-format from 0.4.0 to 0.4.4 (#108)
- Bumps [num-format](https://github.com/bcmyers/num-format) from 0.4.0 to 0.4.4.
<details>
<summary>Changelog</summary>
<p><em>Sourced from <a href="https://github.com/bcmyers/num-format/blob/main/CHANGELOG.md">num-format's changelog</a>.</em></p>
<blockquote>
<h1>0.4.3 (2022-10-09)</h1>
<ul>
<li>Add github actions CI</li>
<li>Fix a few clippy lints</li>
</ul>
<h1>0.4.2 (2022-10-09)</h1>
<ul>
<li>Bump 3rd party dependencies</li>
<li>Update <a href="https://github.com/unicode-cldr/cldr-numbers-full">cldr-numbers-full</a>
git submodule to commit <a href="https://github.com/unicode-cldr/cldr-numbers-full/commit/b52a87048985d3052f12d30d05cfe5423ad92709">b52a87048985d3052f12d30d05cfe5423ad92709</a></li>
<li><code>Locale::from_str</code> now supports underscore-delimited locales, e.g. &quot;de_DE&quot;,
in addition to previously supported dash-delimiated locales, e.g &quot;de-DE&quot;</li>
</ul>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/bcmyers/num-format/commit/c2173715e17a48de2e0b453972715f78a8a7594b"><code>c217371</code></a> [num-format] 0.4.4</li>
<li><a href="https://github.com/bcmyers/num-format/commit/a2bdd9b8ce65c49e3b1d9496d29914995dcbfef8"><code>a2bdd9b</code></a> [num-format-windows] 0.4.4</li>
<li><a href="https://github.com/bcmyers/num-format/commit/fc443eb35450cbd7eb702bf8c6f86f71ea0dfebe"><code>fc443eb</code></a> [num-format-windows] Bump bindgen to 0.63.0</li>
<li><a href="https://github.com/bcmyers/num-format/commit/64e977605c1241dfdd397fa79702eb653f74bf1e"><code>64e9776</code></a> [num-format] 0.4.3</li>
<li><a href="https://github.com/bcmyers/num-format/commit/133d20cce170264c38d1f55eba547b4a3eed6af6"><code>133d20c</code></a> [num-format] Add justfile to replace scripts</li>
<li><a href="https://github.com/bcmyers/num-format/commit/91c783bc6c3cc14f3f192a1f351e8aad4460db41"><code>91c783b</code></a> [num-format] Add CI</li>
<li><a href="https://github.com/bcmyers/num-format/commit/b4e8067262f6dc66197469255705c29ae6ba2434"><code>b4e8067</code></a> [num-format-dev] Remove automatic running of rustfmt</li>
<li><a href="https://github.com/bcmyers/num-format/commit/829c5996e547111155c9548a102761f0f2cf7bc6"><code>829c599</code></a> [num-format] Update min rust version in README</li>
<li><a href="https://github.com/bcmyers/num-format/commit/f131d56b5aeee4f8892a8989454accef174c095c"><code>f131d56</code></a> [num-format] Remove refs to travis-ci.org, which no longer functions</li>
<li><a href="https://github.com/bcmyers/num-format/commit/51f2ff10a05dd94a716b8a5c8e772b38d80ae79f"><code>51f2ff1</code></a> [num-format] 0.4.2</li>
<li>Additional commits viewable in <a href="https://github.com/bcmyers/num-format/compare/v0.4.0...v0.4.4">compare view</a></li>
</ul>
</details>
<br />
- 
[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=num-format&package-manager=cargo&previous-version=0.4.0&new-version=0.4.4)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
- Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.
- [//]: # (dependabot-automerge-start)
[//]: # (dependabot-automerge-end)
- ---
- <details>
<summary>Dependabot commands and options</summary>
<br />
- You can trigger Dependabot actions by commenting on this PR:
- `@dependabot rebase` will rebase this PR
- `@dependabot recreate` will recreate this PR, overwriting any edits that have been made to it
- `@dependabot merge` will merge this PR after your CI passes on it
- `@dependabot squash and merge` will squash and merge this PR after your CI passes on it
- `@dependabot cancel merge` will cancel a previously requested merge and block automerging
- `@dependabot reopen` will reopen this PR if it is closed
- `@dependabot close` will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
- `@dependabot ignore this major version` will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
- `@dependabot ignore this minor version` will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
- `@dependabot ignore this dependency` will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- 
</details>

## 0.2.10 (2023-03-17)


- feat: updated to the latest version of clap
- fix(ci): fixed cargo upgrade cmd again
- fix(ci): fix cmd for new cargo-edit version
- fix(ci): re-enable add-and-commit step
- chore(deps): bump serde_yaml from 0.9.13 to 0.9.19 (#105)
- Bumps [serde_yaml](https://github.com/dtolnay/serde-yaml) from 0.9.13 to 0.9.19.
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/dtolnay/serde-yaml/releases">serde_yaml's releases</a>.</em></p>
<blockquote>
<h2>0.9.19</h2>
<ul>
<li>Fix message duplication between serde_yaml::Error's <code>Display</code> and <code>source()</code> (<a href="https://redirect.github.com/dtolnay/serde-yaml/issues/359">#359</a>, <a href="https://redirect.github.com/dtolnay/serde-yaml/issues/360">#360</a>)</li>
</ul>
<h2>0.9.18</h2>
<ul>
<li>Add support for emitting Unicode characters over codepoint U+FFFF (<a href="https://redirect.github.com/dtolnay/serde-yaml/issues/356">#356</a>)</li>
</ul>
<h2>0.9.17</h2>
<ul>
<li>Improve Debug representation of some error messages</li>
</ul>
<h2>0.9.16</h2>
<ul>
<li>Opt out of <code>-Zrustdoc-scrape-examples</code> on docs.rs for now</li>
</ul>
<h2>0.9.15</h2>
<ul>
<li>Documentation improvements</li>
</ul>
<h2>0.9.14</h2>
<ul>
<li>Implement <code>Deserializer</code> for <code>TaggedValue</code> and <code>&amp;TaggedValue</code> (<a href="https://redirect.github.com/dtolnay/serde-yaml/issues/339">#339</a>)</li>
</ul>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/dtolnay/serde-yaml/commit/4e1cbd8d2feda364b0358e19e19a23e442208f89"><code>4e1cbd8</code></a> Release 0.9.19</li>
<li><a href="https://github.com/dtolnay/serde-yaml/commit/f351fc7a124a0c5176cbdf98fcd2537df5902cbc"><code>f351fc7</code></a> Merge pull request <a href="https://redirect.github.com/dtolnay/serde-yaml/issues/360">#360</a> from dtolnay/errorsource</li>
<li><a href="https://github.com/dtolnay/serde-yaml/commit/f27e4c5da3122a15d7be19d721a220516eeadcd2"><code>f27e4c5</code></a> Fix message duplication between error Display and source()</li>
<li><a href="https://github.com/dtolnay/serde-yaml/commit/fc039c635774e72072bda044f75e53b8877e2004"><code>fc039c6</code></a> Refer to std::error::Error trait as StdError</li>
<li><a href="https://github.com/dtolnay/serde-yaml/commit/7f1db12843a72e797cdaf3f49506c19b51659a64"><code>7f1db12</code></a> Release 0.9.18</li>
<li><a href="https://github.com/dtolnay/serde-yaml/commit/248d6de914b912695f6613ed0d4d0035d82e893e"><code>248d6de</code></a> Merge pull request <a href="https://redirect.github.com/dtolnay/serde-yaml/issues/358">#358</a> from dtolnay/unicode</li>
<li><a href="https://github.com/dtolnay/serde-yaml/commit/779f01676b3ac07a9bc8624d1f79c814b776ba6d"><code>779f016</code></a> Update U+1F389 test</li>
<li><a href="https://github.com/dtolnay/serde-yaml/commit/12b48b5547213f0c6179e4a666fb29caa46bc453"><code>12b48b5</code></a> Pull in Unicode high codepoints fix from unsafe-libyaml 0.2.7</li>
<li><a href="https://github.com/dtolnay/serde-yaml/commit/b6f69579df44b3d200d8975f862ef92e3e23db61"><code>b6f6957</code></a> Add test of Unicode larger than U+FFFF</li>
<li><a href="https://github.com/dtolnay/serde-yaml/commit/eac69a2a0bda6828e4b79ba3d57967099433e14c"><code>eac69a2</code></a> Merge pull request <a href="https://redirect.github.com/dtolnay/serde-yaml/issues/357">#357</a> from dtolnay/stringescape</li>
<li>Additional commits viewable in <a href="https://github.com/dtolnay/serde-yaml/compare/0.9.13...0.9.19">compare view</a></li>
</ul>
</details>
<br />
- 
[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=serde_yaml&package-manager=cargo&previous-version=0.9.13&new-version=0.9.19)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
- Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.
- [//]: # (dependabot-automerge-start)
[//]: # (dependabot-automerge-end)
- ---
- <details>
<summary>Dependabot commands and options</summary>
<br />
- You can trigger Dependabot actions by commenting on this PR:
- `@dependabot rebase` will rebase this PR
- `@dependabot recreate` will recreate this PR, overwriting any edits that have been made to it
- `@dependabot merge` will merge this PR after your CI passes on it
- `@dependabot squash and merge` will squash and merge this PR after your CI passes on it
- `@dependabot cancel merge` will cancel a previously requested merge and block automerging
- `@dependabot reopen` will reopen this PR if it is closed
- `@dependabot close` will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
- `@dependabot ignore this major version` will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
- `@dependabot ignore this minor version` will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
- `@dependabot ignore this dependency` will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- 
</details>
- chore(deps): bump console from 0.15.2 to 0.15.5 (#106)
- Bumps [console](https://github.com/console-rs/console) from 0.15.2 to 0.15.5.
<details>
<summary>Changelog</summary>
<p><em>Sourced from <a href="https://github.com/console-rs/console/blob/master/CHANGELOG.md">console's changelog</a>.</em></p>
<blockquote>
<h2>0.15.5</h2>
<h3>Enhancements</h3>
<ul>
<li>Removed <code>regex</code> dependency. (<a href="https://redirect.github.com/console-rs/console/issues/153">#153</a>)</li>
<li>Clarified that <code>clicolors-control</code> is no longer used.</li>
<li>Handle non-tty terminals in <code>read_char</code>. (<a href="https://redirect.github.com/console-rs/console/issues/124">#124</a>)</li>
</ul>
<h2>0.15.4</h2>
<h3>Enhancements</h3>
<ul>
<li>Fix for regression where console size was misreported on windows. (<a href="https://redirect.github.com/console-rs/console/issues/151">#151</a>)</li>
</ul>
<h2>0.15.3</h2>
<h3>Enhancements</h3>
<ul>
<li>Dropped <code>terminal_size</code> dependency.</li>
</ul>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/console-rs/console/commit/02fa5c0b428c1a5eb15a9411326bbad0a2b79d3b"><code>02fa5c0</code></a> 0.15.5</li>
<li><a href="https://github.com/console-rs/console/commit/e9969cf21a433f2b4a32a5edd352afefed3d61a6"><code>e9969cf</code></a> Update changelog</li>
<li><a href="https://github.com/console-rs/console/commit/ac6f2e38b11b1bbe0f235b2de3b40f96f96e9ae5"><code>ac6f2e3</code></a> read_char: Handle non-tty terminals explicitly (<a href="https://redirect.github.com/console-rs/console/issues/124">#124</a>)</li>
<li><a href="https://github.com/console-rs/console/commit/357ca1d385a2aa8aac6f434f3f4b560747edcb57"><code>357ca1d</code></a> Remove mention of not used clicolors-control</li>
<li><a href="https://github.com/console-rs/console/commit/b577923076cfb0aae671f40a3f0c6bd0afcedacb"><code>b577923</code></a> Bump actions/checkout from 2 to 3 (<a href="https://redirect.github.com/console-rs/console/issues/147">#147</a>)</li>
<li><a href="https://github.com/console-rs/console/commit/171fc16d898e761a695a78cee9fc100707d99892"><code>171fc16</code></a> Drop remaining usages of <code>regex</code> (<a href="https://redirect.github.com/console-rs/console/issues/153">#153</a>)</li>
<li><a href="https://github.com/console-rs/console/commit/a39d90be11e06170f8bbd1a790025f6e1f624708"><code>a39d90b</code></a> 0.15.4</li>
<li><a href="https://github.com/console-rs/console/commit/edfc4bbb05edb70279c720d28a1ba3cb580cbf7a"><code>edfc4bb</code></a> Fix invalid terminal size being returned on Windows (<a href="https://redirect.github.com/console-rs/console/issues/151">#151</a>)</li>
<li><a href="https://github.com/console-rs/console/commit/1b72810fea0b5de9624553265800732a8fe703b8"><code>1b72810</code></a> 0.15.3</li>
<li><a href="https://github.com/console-rs/console/commit/55b043e746745ddf394295f7153dfa56cb6d548e"><code>55b043e</code></a> run clippy on all targets (<a href="https://redirect.github.com/console-rs/console/issues/141">#141</a>)</li>
<li>Additional commits viewable in <a href="https://github.com/console-rs/console/compare/0.15.2...0.15.5">compare view</a></li>
</ul>
</details>
<br />
- 
[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=console&package-manager=cargo&previous-version=0.15.2&new-version=0.15.5)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
- Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.
- [//]: # (dependabot-automerge-start)
[//]: # (dependabot-automerge-end)
- ---
- <details>
<summary>Dependabot commands and options</summary>
<br />
- You can trigger Dependabot actions by commenting on this PR:
- `@dependabot rebase` will rebase this PR
- `@dependabot recreate` will recreate this PR, overwriting any edits that have been made to it
- `@dependabot merge` will merge this PR after your CI passes on it
- `@dependabot squash and merge` will squash and merge this PR after your CI passes on it
- `@dependabot cancel merge` will cancel a previously requested merge and block automerging
- `@dependabot reopen` will reopen this PR if it is closed
- `@dependabot close` will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually
- `@dependabot ignore this major version` will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)
- `@dependabot ignore this minor version` will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)
- `@dependabot ignore this dependency` will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)
- 
</details>

## 0.2.9 (2023-03-17)


- fix(ci): re-enable cargo publish

## 0.2.8 (2023-03-17)

- a year's worth of dependabot updates

## 0.2.7 (2022-01-26)


- fix: added default-members to workspace to prevent publishing the test api
- chore: re-ran cargo sort
- chore: ran cargo sort
- feat: added environment variable overrides
- chore(deps): bump reqwest from 0.11.8 to 0.11.9
- Signed-off-by: dependabot[bot] <support@github.com>
- ci: dont bump on dependabot updates
- ci(revert): move kodiak toml to repo root
- This reverts commit 86da4fd831202456e747d07fcf709d12b0fc5fea.
- ci: move kodiak toml to repo root
- ci: add doc and ci to cz schema pattern
- ci: update kodiak config
- ci: update cz schema validation to allow for scopes
- chore(deps): bump serde from 1.0.133 to 1.0.135
- Signed-off-by: dependabot[bot] <support@github.com>
- doc: update intro sentence
- doc: fixed a weird sentence
- doc: first pass on readme
- added contributing guide (thanks atom for the baseline)
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
