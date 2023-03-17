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
