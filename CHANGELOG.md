## 0.2.8 (2023-03-17)


- chore(deps): bump csv from 1.1.6 to 1.2.1 (#107)
- Bumps [csv](https://github.com/BurntSushi/rust-csv) from 1.1.6 to 1.2.1.
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/BurntSushi/rust-csv/commit/11b02f46d0739bf004e896888922362566efc194"><code>11b02f4</code></a> 1.2.1</li>
<li><a href="https://github.com/BurntSushi/rust-csv/commit/e3c663eecedc2ce4a5529b9326f7e72ead796435"><code>e3c663e</code></a> api: add IntoInnerError::into_error</li>
<li><a href="https://github.com/BurntSushi/rust-csv/commit/f29c9d7e238d467971f94a38427b2d8e227e3c47"><code>f29c9d7</code></a> doc: update tutorial.rs</li>
<li><a href="https://github.com/BurntSushi/rust-csv/commit/e0a8b8b2908e72fe9cd948453e90110001d02231"><code>e0a8b8b</code></a> readme: fix GitHub Actions badge image link</li>
<li><a href="https://github.com/BurntSushi/rust-csv/commit/fa01b78533a58d6c1087990a79a42c97c6edd069"><code>fa01b78</code></a> 1.2.0</li>
<li><a href="https://github.com/BurntSushi/rust-csv/commit/2ba20b55d91688d5e0e86b204d9fc2990c7e6df9"><code>2ba20b5</code></a> readme: various updates</li>
<li><a href="https://github.com/BurntSushi/rust-csv/commit/521294f87b3e1e286829f85a34f397e007f081ed"><code>521294f</code></a> msrv: set rust-version = 1.60</li>
<li><a href="https://github.com/BurntSushi/rust-csv/commit/31c8ebc49d6134f666a6607783883e50460ed87d"><code>31c8ebc</code></a> readme: updates and set rust-version</li>
<li><a href="https://github.com/BurntSushi/rust-csv/commit/a0e83883e2f862103edf42b130639cba50fb02f3"><code>a0e8388</code></a> deps: drop 'bstr'</li>
<li><a href="https://github.com/BurntSushi/rust-csv/commit/9e1126a3f42574cafd31b2309ce686b66444e865"><code>9e1126a</code></a> cargo: update some fields</li>
<li>Additional commits viewable in <a href="https://github.com/BurntSushi/rust-csv/compare/1.1.6...1.2.1">compare view</a></li>
</ul>
</details>
<br />
- 
[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=csv&package-manager=cargo&previous-version=1.1.6&new-version=1.2.1)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
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
- fix(ci): cz enable push and commit
- fix(ci): update publish to use pre-built github token
- fix(ci): update publish workflow to checkout v3
- fix(ci): update publish workflow to checkout v3
- chore(deps): bump serde from 1.0.145 to 1.0.156 (#104)
- Bumps [serde](https://github.com/serde-rs/serde) from 1.0.145 to 1.0.156.
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/serde-rs/serde/releases">serde's releases</a>.</em></p>
<blockquote>
<h2>v1.0.156</h2>
<ul>
<li>Documentation improvements</li>
</ul>
<h2>v1.0.155</h2>
<ul>
<li>Support <code>Serialize</code> and <code>Deserialize</code> impls for <code>core::ffi::CStr</code> and <code>alloc::ffi::CString</code> without &quot;std&quot; feature (<a href="https://redirect.github.com/serde-rs/serde/issues/2374">#2374</a>, thanks <a href="https://github.com/safarir"><code>@​safarir</code></a>)</li>
</ul>
<h2>v1.0.154</h2>
<ul>
<li>Fix &quot;undeclared lifetime&quot; error in generated code when deriving Deserialize for an enum with both <code>flatten</code> and <code>'static</code> fields (<a href="https://redirect.github.com/serde-rs/serde/issues/2383">#2383</a>, thanks <a href="https://github.com/Mingun"><code>@​Mingun</code></a>)</li>
</ul>
<h2>v1.0.153</h2>
<ul>
<li>Support <code>serde(alias = &quot;…&quot;)</code> attribute used inside of flattened struct (<a href="https://redirect.github.com/serde-rs/serde/issues/2387">#2387</a>, thanks <a href="https://github.com/bebecue"><code>@​bebecue</code></a>)</li>
</ul>
<h2>v1.0.152</h2>
<ul>
<li>Documentation improvements</li>
</ul>
<h2>v1.0.151</h2>
<ul>
<li>Update <code>serde::</code>{<code>ser</code>,<code>de</code>}<code>::StdError</code> to re-export <code>core::error::Error</code> when serde is built with <code>feature=&quot;std&quot;</code> <strong>off</strong> and <code>feature=&quot;unstable&quot;</code> <strong>on</strong> (<a href="https://redirect.github.com/serde-rs/serde/issues/2344">#2344</a>)</li>
</ul>
<h2>v1.0.150</h2>
<ul>
<li>Relax some trait bounds from the <code>Serialize</code> impl of <code>HashMap</code> and <code>BTreeMap</code> (<a href="https://redirect.github.com/serde-rs/serde/issues/2334">#2334</a>)</li>
<li>Enable <code>Serialize</code> and <code>Deserialize</code> impls of <code>std::sync::atomic</code> types on more platforms (<a href="https://redirect.github.com/serde-rs/serde/issues/2337">#2337</a>, thanks <a href="https://github.com/badboy"><code>@​badboy</code></a>)</li>
</ul>
<h2>v1.0.149</h2>
<ul>
<li>Relax some trait bounds from the <code>Serialize</code> impl of <code>BinaryHeap</code>, <code>BTreeSet</code>, and <code>HashSet</code> (<a href="https://redirect.github.com/serde-rs/serde/issues/2333">#2333</a>, thanks <a href="https://github.com/jonasbb"><code>@​jonasbb</code></a>)</li>
</ul>
<h2>v1.0.148</h2>
<ul>
<li>Support <code>remote</code> derive for generic types that have private fields (<a href="https://redirect.github.com/serde-rs/serde/issues/2327">#2327</a>)</li>
</ul>
<h2>v1.0.147</h2>
<ul>
<li>Add <code>serde::de::value::EnumAccessDeserializer</code> which transforms an <code>EnumAccess</code> into a <code>Deserializer</code> (<a href="https://redirect.github.com/serde-rs/serde/issues/2305">#2305</a>)</li>
</ul>
<h2>v1.0.146</h2>
<ul>
<li>Allow internally tagged newtype variant to contain unit (<a href="https://redirect.github.com/serde-rs/serde/issues/2303">#2303</a>, thanks <a href="https://github.com/tage64"><code>@​tage64</code></a>)</li>
</ul>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/serde-rs/serde/commit/54671259aaaf2617b0f27dce6cc519058d4f3a90"><code>5467125</code></a> Release 1.0.156</li>
<li><a href="https://github.com/serde-rs/serde/commit/994f7c7924f7fccde5c474644f6d22115586bdee"><code>994f7c7</code></a> Format with rustfmt 1.5.2-nightly</li>
<li><a href="https://github.com/serde-rs/serde/commit/7a8e4977e230e2da45bb7e1f75d7a3175b6a7755"><code>7a8e497</code></a> Merge pull request <a href="https://redirect.github.com/serde-rs/serde/issues/2401">#2401</a> from dtolnay/docderive</li>
<li><a href="https://github.com/serde-rs/serde/commit/fb7b6ea7ea8864a41e78378ca0555ce3f1dd8965"><code>fb7b6ea</code></a> Enable serde derive feature when built by docs.rs</li>
<li><a href="https://github.com/serde-rs/serde/commit/063dd5b93f9f0c3181de399132441668fed67029"><code>063dd5b</code></a> Show derive macros in serde's rustdoc</li>
<li><a href="https://github.com/serde-rs/serde/commit/a38aa31ade469b034dc8d04c39cc52c0e76c9e75"><code>a38aa31</code></a> Merge pull request <a href="https://redirect.github.com/serde-rs/serde/issues/2400">#2400</a> from Nilstrieb/explicit-reexport</li>
<li><a href="https://github.com/serde-rs/serde/commit/f42b2581da2bdbb81713a93d3b7e581b81e4332e"><code>f42b258</code></a> Use explicit re-export of <code>serde_derive</code> to give rustc more info</li>
<li><a href="https://github.com/serde-rs/serde/commit/2ba406726f9f84bc3b65ce4e824ae636dfa7dc85"><code>2ba4067</code></a> Release 1.0.155</li>
<li><a href="https://github.com/serde-rs/serde/commit/7e9826e17b154b32742b71285349ec042da7ee35"><code>7e9826e</code></a> Add link to core CStr stabilization announcement</li>
<li><a href="https://github.com/serde-rs/serde/commit/f4dcc5c918a54712fbf76f50883c6c496d0b8af9"><code>f4dcc5c</code></a> Merge pull request <a href="https://redirect.github.com/serde-rs/serde/issues/2374">#2374</a> from safarir/master</li>
<li>Additional commits viewable in <a href="https://github.com/serde-rs/serde/compare/v1.0.145...v1.0.156">compare view</a></li>
</ul>
</details>
<br />
- 
[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=serde&package-manager=cargo&previous-version=1.0.145&new-version=1.0.156)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
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
- chore(deps): bump serde_json from 1.0.93 to 1.0.94 (#103)
- Bumps [serde_json](https://github.com/serde-rs/json) from 1.0.93 to 1.0.94.
<details>
<summary>Release notes</summary>
<p><em>Sourced from <a href="https://github.com/serde-rs/json/releases">serde_json's releases</a>.</em></p>
<blockquote>
<h2>v1.0.94</h2>
<ul>
<li>Fix message duplication between serde_json::Error's <code>Display</code> and <code>source()</code> (<a href="https://redirect.github.com/serde-rs/json/issues/991">#991</a>, <a href="https://redirect.github.com/serde-rs/json/issues/992">#992</a>)</li>
</ul>
</blockquote>
</details>
<details>
<summary>Commits</summary>
<ul>
<li><a href="https://github.com/serde-rs/json/commit/a15bd0968639884ec7b73107360d58fd655e2071"><code>a15bd09</code></a> Release 1.0.94</li>
<li><a href="https://github.com/serde-rs/json/commit/3e418b13be142ca5c484eb1db6b3a02e2f8121e0"><code>3e418b1</code></a> Merge pull request <a href="https://redirect.github.com/serde-rs/json/issues/992">#992</a> from dtolnay/errorsource</li>
<li><a href="https://github.com/serde-rs/json/commit/7eeb169f9b51e2a30997d6c92aa3e170a2927b7f"><code>7eeb169</code></a> Fix message duplication between error Display and source()</li>
<li><a href="https://github.com/serde-rs/json/commit/d9447c30eb0ff682923499dfb18fb229d5dea84d"><code>d9447c3</code></a> Ignore let_underscore_untyped pedantic clippy lint</li>
<li>See full diff in <a href="https://github.com/serde-rs/json/compare/v1.0.93...v1.0.94">compare view</a></li>
</ul>
</details>
<br />
- 
[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=serde_json&package-manager=cargo&previous-version=1.0.93&new-version=1.0.94)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
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
- chore(ci): added automerge label to dependabot PRs
- chore(ci): added automerge to kodiak for dependabot
- Merge pull request #95 from pdylanross/dependabot/cargo/reqwest-0.11.14
- chore(deps): bump reqwest from 0.11.12 to 0.11.14
- Bumps [reqwest](https://github.com/seanmonstar/reqwest) from 0.11.12 to 0.11.14.
- [Release notes](https://github.com/seanmonstar/reqwest/releases)
- [Changelog](https://github.com/seanmonstar/reqwest/blob/master/CHANGELOG.md)
- [Commits](https://github.com/seanmonstar/reqwest/compare/v0.11.12...v0.11.14)
- ---
updated-dependencies:
- dependency-name: reqwest
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #97 from pdylanross/dependabot/cargo/crossterm-0.26.1
- chore(deps): bump crossterm from 0.25.0 to 0.26.1
- Bumps [crossterm](https://github.com/crossterm-rs/crossterm) from 0.25.0 to 0.26.1.
- [Release notes](https://github.com/crossterm-rs/crossterm/releases)
- [Changelog](https://github.com/crossterm-rs/crossterm/blob/master/CHANGELOG.md)
- [Commits](https://github.com/crossterm-rs/crossterm/compare/0.25...0.26.1)
- ---
updated-dependencies:
- dependency-name: crossterm
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #93 from pdylanross/dependabot/cargo/prettytable-rs-0.10.0
- chore(deps): bump prettytable-rs from 0.9.0 to 0.10.0
- Bumps [prettytable-rs](https://github.com/phsym/prettytable-rs) from 0.9.0 to 0.10.0.
- [Release notes](https://github.com/phsym/prettytable-rs/releases)
- [Changelog](https://github.com/phsym/prettytable-rs/blob/master/CHANGELOG.md)
- [Commits](https://github.com/phsym/prettytable-rs/compare/v0.9.0...v0.10.0)
- ---
updated-dependencies:
- dependency-name: prettytable-rs
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #98 from pdylanross/dependabot/cargo/thiserror-1.0.39
- chore(deps): bump thiserror from 1.0.37 to 1.0.39
- Bumps [thiserror](https://github.com/dtolnay/thiserror) from 1.0.37 to 1.0.39.
- [Release notes](https://github.com/dtolnay/thiserror/releases)
- [Commits](https://github.com/dtolnay/thiserror/compare/1.0.37...1.0.39)
- ---
updated-dependencies:
- dependency-name: thiserror
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #77 from pdylanross/dependabot/cargo/serde_yaml-0.9.13
- fix: addressed serde_yaml breaking change
- chore(deps): bump serde_yaml from 0.8.26 to 0.9.13
- Bumps [serde_yaml](https://github.com/dtolnay/serde-yaml) from 0.8.26 to 0.9.13.
- [Release notes](https://github.com/dtolnay/serde-yaml/releases)
- [Commits](https://github.com/dtolnay/serde-yaml/compare/0.8.26...0.9.13)
- ---
updated-dependencies:
- dependency-name: serde_yaml
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #90 from pdylanross/dependabot/cargo/futures-0.3.26
- chore(deps): bump futures from 0.3.24 to 0.3.26
- Bumps [futures](https://github.com/rust-lang/futures-rs) from 0.3.24 to 0.3.26.
- [Release notes](https://github.com/rust-lang/futures-rs/releases)
- [Changelog](https://github.com/rust-lang/futures-rs/blob/master/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/futures-rs/compare/0.3.24...0.3.26)
- ---
updated-dependencies:
- dependency-name: futures
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #89 from pdylanross/dependabot/cargo/uuid-1.3.0
- chore(deps): bump uuid from 1.1.2 to 1.3.0
- Bumps [uuid](https://github.com/uuid-rs/uuid) from 1.1.2 to 1.3.0.
- [Release notes](https://github.com/uuid-rs/uuid/releases)
- [Commits](https://github.com/uuid-rs/uuid/compare/1.1.2...1.3.0)
- ---
updated-dependencies:
- dependency-name: uuid
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #82 from pdylanross/dependabot/cargo/figment-0.10.8
- chore(deps): bump figment from 0.10.7 to 0.10.8
- Bumps [figment](https://github.com/SergioBenitez/Figment) from 0.10.7 to 0.10.8.
- [Release notes](https://github.com/SergioBenitez/Figment/releases)
- [Commits](https://github.com/SergioBenitez/Figment/compare/v0.10.7...v0.10.8)
- ---
updated-dependencies:
- dependency-name: figment
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #92 from pdylanross/dependabot/cargo/serde_json-1.0.93
- chore(deps): bump serde_json from 1.0.85 to 1.0.93
- Bumps [serde_json](https://github.com/serde-rs/json) from 1.0.85 to 1.0.93.
- [Release notes](https://github.com/serde-rs/json/releases)
- [Commits](https://github.com/serde-rs/json/compare/v1.0.85...v1.0.93)
- ---
updated-dependencies:
- dependency-name: serde_json
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #75 from pdylanross/dependabot/cargo/itertools-0.10.5
- chore(deps): bump itertools from 0.10.4 to 0.10.5
- Bumps [itertools](https://github.com/rust-itertools/itertools) from 0.10.4 to 0.10.5.
- [Release notes](https://github.com/rust-itertools/itertools/releases)
- [Changelog](https://github.com/rust-itertools/itertools/blob/master/CHANGELOG.md)
- [Commits](https://github.com/rust-itertools/itertools/commits)
- ---
updated-dependencies:
- dependency-name: itertools
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #79 from pdylanross/dependabot/cargo/console-0.15.2
- chore(deps): bump console from 0.15.1 to 0.15.2
- Bumps [console](https://github.com/console-rs/console) from 0.15.1 to 0.15.2.
- [Release notes](https://github.com/console-rs/console/releases)
- [Changelog](https://github.com/console-rs/console/blob/master/CHANGELOG.md)
- [Commits](https://github.com/console-rs/console/compare/0.15.1...0.15.2)
- ---
updated-dependencies:
- dependency-name: console
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #74 from pdylanross/dependabot/cargo/reqwest-0.11.12
- chore(deps): bump reqwest from 0.11.11 to 0.11.12
- Bumps [reqwest](https://github.com/seanmonstar/reqwest) from 0.11.11 to 0.11.12.
- [Release notes](https://github.com/seanmonstar/reqwest/releases)
- [Changelog](https://github.com/seanmonstar/reqwest/blob/master/CHANGELOG.md)
- [Commits](https://github.com/seanmonstar/reqwest/compare/v0.11.11...v0.11.12)
- ---
updated-dependencies:
- dependency-name: reqwest
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #73 from pdylanross/dependabot/cargo/serde-1.0.145
- chore(deps): bump serde from 1.0.144 to 1.0.145
- Bumps [serde](https://github.com/serde-rs/serde) from 1.0.144 to 1.0.145.
- [Release notes](https://github.com/serde-rs/serde/releases)
- [Commits](https://github.com/serde-rs/serde/compare/v1.0.144...v1.0.145)
- ---
updated-dependencies:
- dependency-name: serde
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #78 from pdylanross/dependabot/cargo/thiserror-1.0.37
- chore(deps): bump thiserror from 1.0.35 to 1.0.37
- Bumps [thiserror](https://github.com/dtolnay/thiserror) from 1.0.35 to 1.0.37.
- [Release notes](https://github.com/dtolnay/thiserror/releases)
- [Commits](https://github.com/dtolnay/thiserror/compare/1.0.35...1.0.37)
- ---
updated-dependencies:
- dependency-name: thiserror
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #69 from pdylanross/dependabot/cargo/thiserror-1.0.35
- chore(deps): bump thiserror from 1.0.34 to 1.0.35
- Bumps [thiserror](https://github.com/dtolnay/thiserror) from 1.0.34 to 1.0.35.
- [Release notes](https://github.com/dtolnay/thiserror/releases)
- [Commits](https://github.com/dtolnay/thiserror/compare/1.0.34...1.0.35)
- ---
updated-dependencies:
- dependency-name: thiserror
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #67 from pdylanross/dependabot/cargo/figment-0.10.7
- chore(deps): bump figment from 0.10.6 to 0.10.7
- Bumps [figment](https://github.com/SergioBenitez/Figment) from 0.10.6 to 0.10.7.
- [Release notes](https://github.com/SergioBenitez/Figment/releases)
- [Commits](https://github.com/SergioBenitez/Figment/compare/v0.10.6...v0.10.7)
- ---
updated-dependencies:
- dependency-name: figment
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #71 from pdylanross/dependabot/cargo/url-2.3.1
- chore(deps): bump url from 2.3.0 to 2.3.1
- Bumps [url](https://github.com/servo/rust-url) from 2.3.0 to 2.3.1.
- [Release notes](https://github.com/servo/rust-url/releases)
- [Commits](https://github.com/servo/rust-url/compare/v2.3.0...v2.3.1)
- ---
updated-dependencies:
- dependency-name: url
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #70 from pdylanross/dependabot/cargo/hdrhistogram-7.5.2
- chore(deps): bump hdrhistogram from 7.5.1 to 7.5.2
- Bumps [hdrhistogram](https://github.com/HdrHistogram/HdrHistogram_rust) from 7.5.1 to 7.5.2.
- [Release notes](https://github.com/HdrHistogram/HdrHistogram_rust/releases)
- [Changelog](https://github.com/HdrHistogram/HdrHistogram_rust/blob/main/CHANGELOG.md)
- [Commits](https://github.com/HdrHistogram/HdrHistogram_rust/compare/v7.5.1...v7.5.2)
- ---
updated-dependencies:
- dependency-name: hdrhistogram
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #72 from pdylanross/dependabot/cargo/itertools-0.10.4
- chore(deps): bump itertools from 0.10.3 to 0.10.4
- Bumps [itertools](https://github.com/rust-itertools/itertools) from 0.10.3 to 0.10.4.
- [Release notes](https://github.com/rust-itertools/itertools/releases)
- [Changelog](https://github.com/rust-itertools/itertools/blob/master/CHANGELOG.md)
- [Commits](https://github.com/rust-itertools/itertools/compare/v0.10.3...v0.10.4)
- ---
updated-dependencies:
- dependency-name: itertools
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #66 from pdylanross/dependabot/cargo/thiserror-1.0.34
- chore(deps): bump thiserror from 1.0.32 to 1.0.34
- Bumps [thiserror](https://github.com/dtolnay/thiserror) from 1.0.32 to 1.0.34.
- [Release notes](https://github.com/dtolnay/thiserror/releases)
- [Commits](https://github.com/dtolnay/thiserror/compare/1.0.32...1.0.34)
- ---
updated-dependencies:
- dependency-name: thiserror
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #65 from pdylanross/dependabot/cargo/futures-0.3.24
- chore(deps): bump futures from 0.3.23 to 0.3.24
- Bumps [futures](https://github.com/rust-lang/futures-rs) from 0.3.23 to 0.3.24.
- [Release notes](https://github.com/rust-lang/futures-rs/releases)
- [Changelog](https://github.com/rust-lang/futures-rs/blob/master/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/futures-rs/compare/0.3.23...0.3.24)
- ---
updated-dependencies:
- dependency-name: futures
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #68 from pdylanross/dependabot/cargo/url-2.3.0
- chore(deps): bump url from 2.2.2 to 2.3.0
- Bumps [url](https://github.com/servo/rust-url) from 2.2.2 to 2.3.0.
- [Release notes](https://github.com/servo/rust-url/releases)
- [Commits](https://github.com/servo/rust-url/compare/v2.2.2...v2.3.0)
- ---
updated-dependencies:
- dependency-name: url
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #61 from pdylanross/dependabot/cargo/crossterm-0.25.0
- chore(deps): bump crossterm from 0.24.0 to 0.25.0
- Bumps [crossterm](https://github.com/crossterm-rs/crossterm) from 0.24.0 to 0.25.0.
- [Release notes](https://github.com/crossterm-rs/crossterm/releases)
- [Changelog](https://github.com/crossterm-rs/crossterm/blob/master/CHANGELOG.md)
- [Commits](https://github.com/crossterm-rs/crossterm/compare/0.24...0.25)
- ---
updated-dependencies:
- dependency-name: crossterm
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #59 from pdylanross/dependabot/cargo/futures-0.3.23
- chore(deps): bump futures from 0.3.21 to 0.3.23
- Bumps [futures](https://github.com/rust-lang/futures-rs) from 0.3.21 to 0.3.23.
- [Release notes](https://github.com/rust-lang/futures-rs/releases)
- [Changelog](https://github.com/rust-lang/futures-rs/blob/master/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/futures-rs/compare/0.3.21...0.3.23)
- ---
updated-dependencies:
- dependency-name: futures
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #64 from pdylanross/dependabot/cargo/prettytable-rs-0.9.0
- chore(deps): bump prettytable-rs from 0.8.0 to 0.9.0
- Bumps [prettytable-rs](https://github.com/phsym/prettytable-rs) from 0.8.0 to 0.9.0.
- [Release notes](https://github.com/phsym/prettytable-rs/releases)
- [Commits](https://github.com/phsym/prettytable-rs/compare/v0.8.0...v0.9.0)
- ---
updated-dependencies:
- dependency-name: prettytable-rs
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #60 from pdylanross/dependabot/cargo/hdrhistogram-7.5.1
- chore(deps): bump hdrhistogram from 7.5.0 to 7.5.1
- Bumps [hdrhistogram](https://github.com/HdrHistogram/HdrHistogram_rust) from 7.5.0 to 7.5.1.
- [Release notes](https://github.com/HdrHistogram/HdrHistogram_rust/releases)
- [Changelog](https://github.com/HdrHistogram/HdrHistogram_rust/blob/master/CHANGELOG.md)
- [Commits](https://github.com/HdrHistogram/HdrHistogram_rust/compare/v7.5.0...v7.5.1)
- ---
updated-dependencies:
- dependency-name: hdrhistogram
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #62 from pdylanross/dependabot/cargo/serde_json-1.0.85
- chore(deps): bump serde_json from 1.0.83 to 1.0.85
- Bumps [serde_json](https://github.com/serde-rs/json) from 1.0.83 to 1.0.85.
- [Release notes](https://github.com/serde-rs/json/releases)
- [Commits](https://github.com/serde-rs/json/compare/v1.0.83...v1.0.85)
- ---
updated-dependencies:
- dependency-name: serde_json
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #63 from pdylanross/dependabot/cargo/serde-1.0.144
- chore(deps): bump serde from 1.0.142 to 1.0.144
- Bumps [serde](https://github.com/serde-rs/serde) from 1.0.142 to 1.0.144.
- [Release notes](https://github.com/serde-rs/serde/releases)
- [Commits](https://github.com/serde-rs/serde/compare/v1.0.142...v1.0.144)
- ---
updated-dependencies:
- dependency-name: serde
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #54 from pdylanross/dependabot/cargo/serde-1.0.142
- chore(deps): bump serde from 1.0.137 to 1.0.142
- Bumps [serde](https://github.com/serde-rs/serde) from 1.0.137 to 1.0.142.
- [Release notes](https://github.com/serde-rs/serde/releases)
- [Commits](https://github.com/serde-rs/serde/compare/v1.0.137...v1.0.142)
- ---
updated-dependencies:
- dependency-name: serde
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #55 from pdylanross/dependabot/cargo/serde_json-1.0.83
- chore(deps): bump serde_json from 1.0.82 to 1.0.83
- Bumps [serde_json](https://github.com/serde-rs/json) from 1.0.82 to 1.0.83.
- [Release notes](https://github.com/serde-rs/json/releases)
- [Commits](https://github.com/serde-rs/json/compare/v1.0.82...v1.0.83)
- ---
updated-dependencies:
- dependency-name: serde_json
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #56 from pdylanross/dependabot/cargo/console-0.15.1
- chore(deps): bump console from 0.15.0 to 0.15.1
- Bumps [console](https://github.com/mitsuhiko/console) from 0.15.0 to 0.15.1.
- [Release notes](https://github.com/mitsuhiko/console/releases)
- [Changelog](https://github.com/console-rs/console/blob/master/CHANGELOG.md)
- [Commits](https://github.com/mitsuhiko/console/compare/0.15.0...0.15.1)
- ---
updated-dependencies:
- dependency-name: console
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #57 from pdylanross/dependabot/cargo/thiserror-1.0.32
- chore(deps): bump thiserror from 1.0.31 to 1.0.32
- Bumps [thiserror](https://github.com/dtolnay/thiserror) from 1.0.31 to 1.0.32.
- [Release notes](https://github.com/dtolnay/thiserror/releases)
- [Commits](https://github.com/dtolnay/thiserror/compare/1.0.31...1.0.32)
- ---
updated-dependencies:
- dependency-name: thiserror
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #47 from pdylanross/dependabot/cargo/crossterm-0.24.0
- chore(deps): bump crossterm from 0.23.2 to 0.24.0
- Bumps [crossterm](https://github.com/crossterm-rs/crossterm) from 0.23.2 to 0.24.0.
- [Release notes](https://github.com/crossterm-rs/crossterm/releases)
- [Changelog](https://github.com/crossterm-rs/crossterm/blob/master/CHANGELOG.md)
- [Commits](https://github.com/crossterm-rs/crossterm/commits/0.24)
- ---
updated-dependencies:
- dependency-name: crossterm
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #50 from pdylanross/dependabot/cargo/serde_yaml-0.8.26
- chore(deps): bump serde_yaml from 0.8.24 to 0.8.26
- Bumps [serde_yaml](https://github.com/dtolnay/serde-yaml) from 0.8.24 to 0.8.26.
- [Release notes](https://github.com/dtolnay/serde-yaml/releases)
- [Commits](https://github.com/dtolnay/serde-yaml/compare/0.8.24...0.8.26)
- ---
updated-dependencies:
- dependency-name: serde_yaml
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #52 from pdylanross/dependabot/cargo/async-trait-0.1.57
- chore(deps): bump async-trait from 0.1.56 to 0.1.57
- Bumps [async-trait](https://github.com/dtolnay/async-trait) from 0.1.56 to 0.1.57.
- [Release notes](https://github.com/dtolnay/async-trait/releases)
- [Commits](https://github.com/dtolnay/async-trait/compare/0.1.56...0.1.57)
- ---
updated-dependencies:
- dependency-name: async-trait
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #46 from pdylanross/dependabot/cargo/serde_json-1.0.82
- chore(deps): bump serde_json from 1.0.81 to 1.0.82
- Bumps [serde_json](https://github.com/serde-rs/json) from 1.0.81 to 1.0.82.
- [Release notes](https://github.com/serde-rs/json/releases)
- [Commits](https://github.com/serde-rs/json/compare/v1.0.81...v1.0.82)
- ---
updated-dependencies:
- dependency-name: serde_json
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #41 from pdylanross/dependabot/cargo/async-trait-0.1.56
- chore(deps): bump async-trait from 0.1.53 to 0.1.56
- Bumps [async-trait](https://github.com/dtolnay/async-trait) from 0.1.53 to 0.1.56.
- [Release notes](https://github.com/dtolnay/async-trait/releases)
- [Commits](https://github.com/dtolnay/async-trait/compare/0.1.53...0.1.56)
- ---
updated-dependencies:
- dependency-name: async-trait
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #43 from pdylanross/dependabot/cargo/uuid-1.1.2
- chore(deps): bump uuid from 1.0.0 to 1.1.2
- Bumps [uuid](https://github.com/uuid-rs/uuid) from 1.0.0 to 1.1.2.
- [Release notes](https://github.com/uuid-rs/uuid/releases)
- [Commits](https://github.com/uuid-rs/uuid/compare/1.0.0...1.1.2)
- ---
updated-dependencies:
- dependency-name: uuid
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #44 from pdylanross/dependabot/cargo/reqwest-0.11.11
- chore(deps): bump reqwest from 0.11.10 to 0.11.11
- Bumps [reqwest](https://github.com/seanmonstar/reqwest) from 0.11.10 to 0.11.11.
- [Release notes](https://github.com/seanmonstar/reqwest/releases)
- [Changelog](https://github.com/seanmonstar/reqwest/blob/master/CHANGELOG.md)
- [Commits](https://github.com/seanmonstar/reqwest/compare/v0.11.10...v0.11.11)
- ---
updated-dependencies:
- dependency-name: reqwest
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #39 from pdylanross/dependabot/cargo/rocket-0.5.0-rc.2
- chore(deps): bump rocket from 0.5.0-rc.1 to 0.5.0-rc.2
- Bumps [rocket](https://github.com/SergioBenitez/Rocket) from 0.5.0-rc.1 to 0.5.0-rc.2.
- [Release notes](https://github.com/SergioBenitez/Rocket/releases)
- [Changelog](https://github.com/SergioBenitez/Rocket/blob/master/CHANGELOG.md)
- [Commits](https://github.com/SergioBenitez/Rocket/compare/v0.5.0-rc.1...v0.5.0-rc.2)
- ---
updated-dependencies:
- dependency-name: rocket
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #36 from pdylanross/dependabot/cargo/thiserror-1.0.31
- chore(deps): bump thiserror from 1.0.30 to 1.0.31
- Bumps [thiserror](https://github.com/dtolnay/thiserror) from 1.0.30 to 1.0.31.
- [Release notes](https://github.com/dtolnay/thiserror/releases)
- [Commits](https://github.com/dtolnay/thiserror/compare/1.0.30...1.0.31)
- ---
updated-dependencies:
- dependency-name: thiserror
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #35 from pdylanross/dependabot/cargo/serde-1.0.137
- chore(deps): bump serde from 1.0.136 to 1.0.137
- Bumps [serde](https://github.com/serde-rs/serde) from 1.0.136 to 1.0.137.
- [Release notes](https://github.com/serde-rs/serde/releases)
- [Commits](https://github.com/serde-rs/serde/compare/v1.0.136...v1.0.137)
- ---
updated-dependencies:
- dependency-name: serde
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #37 from pdylanross/dependabot/cargo/serde_json-1.0.81
- chore(deps): bump serde_json from 1.0.79 to 1.0.81
- Bumps [serde_json](https://github.com/serde-rs/json) from 1.0.79 to 1.0.81.
- [Release notes](https://github.com/serde-rs/json/releases)
- [Commits](https://github.com/serde-rs/json/compare/v1.0.79...v1.0.81)
- ---
updated-dependencies:
- dependency-name: serde_json
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #38 from pdylanross/dependabot/cargo/serde_yaml-0.8.24
- chore(deps): bump serde_yaml from 0.8.23 to 0.8.24
- Bumps [serde_yaml](https://github.com/dtolnay/serde-yaml) from 0.8.23 to 0.8.24.
- [Release notes](https://github.com/dtolnay/serde-yaml/releases)
- [Commits](https://github.com/dtolnay/serde-yaml/compare/0.8.23...0.8.24)
- ---
updated-dependencies:
- dependency-name: serde_yaml
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #33 from pdylanross/dependabot/cargo/uuid-1.0.0
- chore(deps): bump uuid from 0.8.2 to 1.0.0
- Bumps [uuid](https://github.com/uuid-rs/uuid) from 0.8.2 to 1.0.0.
- [Release notes](https://github.com/uuid-rs/uuid/releases)
- [Commits](https://github.com/uuid-rs/uuid/compare/0.8.2...1.0.0)
- ---
updated-dependencies:
- dependency-name: uuid
  dependency-type: direct:production
  update-type: version-update:semver-major
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #31 from pdylanross/dependabot/cargo/crossterm-0.23.2
- chore(deps): bump crossterm from 0.23.0 to 0.23.2
- Bumps [crossterm](https://github.com/crossterm-rs/crossterm) from 0.23.0 to 0.23.2.
- [Release notes](https://github.com/crossterm-rs/crossterm/releases)
- [Changelog](https://github.com/crossterm-rs/crossterm/blob/master/CHANGELOG.md)
- [Commits](https://github.com/crossterm-rs/crossterm/commits)
- ---
updated-dependencies:
- dependency-name: crossterm
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #27 from pdylanross/dependabot/cargo/reqwest-0.11.10
- chore(deps): bump reqwest from 0.11.9 to 0.11.10
- Bumps [reqwest](https://github.com/seanmonstar/reqwest) from 0.11.9 to 0.11.10.
- [Release notes](https://github.com/seanmonstar/reqwest/releases)
- [Changelog](https://github.com/seanmonstar/reqwest/blob/master/CHANGELOG.md)
- [Commits](https://github.com/seanmonstar/reqwest/compare/v0.11.9...v0.11.10)
- ---
updated-dependencies:
- dependency-name: reqwest
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #30 from pdylanross/dependabot/cargo/async-trait-0.1.53
- chore(deps): bump async-trait from 0.1.52 to 0.1.53
- Bumps [async-trait](https://github.com/dtolnay/async-trait) from 0.1.52 to 0.1.53.
- [Release notes](https://github.com/dtolnay/async-trait/releases)
- [Commits](https://github.com/dtolnay/async-trait/compare/0.1.52...0.1.53)
- ---
updated-dependencies:
- dependency-name: async-trait
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #28 from pdylanross/dependabot/cargo/dashmap-5.2.0
- chore(deps): bump dashmap from 5.1.0 to 5.2.0
- Bumps [dashmap](https://github.com/xacrimon/dashmap) from 5.1.0 to 5.2.0.
- [Release notes](https://github.com/xacrimon/dashmap/releases)
- [Commits](https://github.com/xacrimon/dashmap/compare/v5.1.0...v5.2.0)
- ---
updated-dependencies:
- dependency-name: dashmap
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #32 from pdylanross/dependabot/cargo/liquid-0.26.0
- chore(deps): bump liquid from 0.24.0 to 0.26.0
- Bumps [liquid](https://github.com/cobalt-org/liquid-rust) from 0.24.0 to 0.26.0.
- [Release notes](https://github.com/cobalt-org/liquid-rust/releases)
- [Changelog](https://github.com/cobalt-org/liquid-rust/blob/master/CHANGELOG.md)
- [Commits](https://github.com/cobalt-org/liquid-rust/compare/liquid-bin-v0.24.0...liquid-bin-v0.26.0)
- ---
updated-dependencies:
- dependency-name: liquid
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #26 from pdylanross/dependabot/cargo/humantime-serde-1.1.1
- chore(deps): bump humantime-serde from 1.0.1 to 1.1.1
- Bumps [humantime-serde](https://github.com/jean-airoldie/humantime-serde) from 1.0.1 to 1.1.1.
- [Release notes](https://github.com/jean-airoldie/humantime-serde/releases)
- [Commits](https://github.com/jean-airoldie/humantime-serde/commits)
- ---
updated-dependencies:
- dependency-name: humantime-serde
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #25 from pdylanross/dependabot/cargo/liquid-0.24.0
- chore(deps): bump liquid from 0.23.1 to 0.24.0
- Bumps [liquid](https://github.com/cobalt-org/liquid-rust) from 0.23.1 to 0.24.0.
- [Release notes](https://github.com/cobalt-org/liquid-rust/releases)
- [Changelog](https://github.com/cobalt-org/liquid-rust/blob/master/CHANGELOG.md)
- [Commits](https://github.com/cobalt-org/liquid-rust/compare/liquid-bin-v0.23.1...liquid-bin-v0.24.0)
- ---
updated-dependencies:
- dependency-name: liquid
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #24 from pdylanross/dependabot/cargo/rand-0.8.5
- chore(deps): bump rand from 0.8.4 to 0.8.5
- Bumps [rand](https://github.com/rust-random/rand) from 0.8.4 to 0.8.5.
- [Release notes](https://github.com/rust-random/rand/releases)
- [Changelog](https://github.com/rust-random/rand/blob/master/CHANGELOG.md)
- [Commits](https://github.com/rust-random/rand/compare/0.8.4...0.8.5)
- ---
updated-dependencies:
- dependency-name: rand
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #22 from pdylanross/dependabot/cargo/hdrhistogram-7.5.0
- chore(deps): bump hdrhistogram from 7.4.0 to 7.5.0
- Bumps [hdrhistogram](https://github.com/HdrHistogram/HdrHistogram_rust) from 7.4.0 to 7.5.0.
- [Release notes](https://github.com/HdrHistogram/HdrHistogram_rust/releases)
- [Changelog](https://github.com/HdrHistogram/HdrHistogram_rust/blob/master/CHANGELOG.md)
- [Commits](https://github.com/HdrHistogram/HdrHistogram_rust/compare/v7.4.0...v7.5.0)
- ---
updated-dependencies:
- dependency-name: hdrhistogram
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #23 from pdylanross/dependabot/cargo/serde_json-1.0.79
- chore(deps): bump serde_json from 1.0.78 to 1.0.79
- Bumps [serde_json](https://github.com/serde-rs/json) from 1.0.78 to 1.0.79.
- [Release notes](https://github.com/serde-rs/json/releases)
- [Commits](https://github.com/serde-rs/json/compare/v1.0.78...v1.0.79)
- ---
updated-dependencies:
- dependency-name: serde_json
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #19 from pdylanross/dependabot/cargo/dashmap-5.1.0
- chore(deps): bump dashmap from 5.0.0 to 5.1.0
- Bumps [dashmap](https://github.com/xacrimon/dashmap) from 5.0.0 to 5.1.0.
- [Release notes](https://github.com/xacrimon/dashmap/releases)
- [Commits](https://github.com/xacrimon/dashmap/compare/v5.0.0...v5.1.0)
- ---
updated-dependencies:
- dependency-name: dashmap
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #20 from pdylanross/dependabot/cargo/crossterm-0.23.0
- chore(deps): bump crossterm from 0.22.1 to 0.23.0
- Bumps [crossterm](https://github.com/crossterm-rs/crossterm) from 0.22.1 to 0.23.0.
- [Release notes](https://github.com/crossterm-rs/crossterm/releases)
- [Changelog](https://github.com/crossterm-rs/crossterm/blob/master/CHANGELOG.md)
- [Commits](https://github.com/crossterm-rs/crossterm/commits/0.23)
- ---
updated-dependencies:
- dependency-name: crossterm
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #21 from pdylanross/dependabot/cargo/futures-0.3.21
- chore(deps): bump futures from 0.3.19 to 0.3.21
- Bumps [futures](https://github.com/rust-lang/futures-rs) from 0.3.19 to 0.3.21.
- [Release notes](https://github.com/rust-lang/futures-rs/releases)
- [Changelog](https://github.com/rust-lang/futures-rs/blob/master/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/futures-rs/compare/0.3.19...0.3.21)
- ---
updated-dependencies:
- dependency-name: futures
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #18 from pdylanross/dependabot/cargo/serde-1.0.136
- chore(deps): bump serde from 1.0.135 to 1.0.136
- Bumps [serde](https://github.com/serde-rs/serde) from 1.0.135 to 1.0.136.
- [Release notes](https://github.com/serde-rs/serde/releases)
- [Commits](https://github.com/serde-rs/serde/compare/v1.0.135...v1.0.136)
- ---
updated-dependencies:
- dependency-name: serde
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
- Signed-off-by: dependabot[bot] <support@github.com>
- Merge pull request #17 from pdylanross/dependabot/cargo/tokio-1.16.1
- chore(deps): bump tokio from 1.15.0 to 1.16.1
- Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.15.0 to 1.16.1.
- [Release notes](https://github.com/tokio-rs/tokio/releases)
- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-1.15.0...tokio-1.16.1)
- ---
updated-dependencies:
- dependency-name: tokio
  dependency-type: direct:production
  update-type: version-update:semver-minor
...
- Signed-off-by: dependabot[bot] <support@github.com>
- ci: remove multi-publish workaround in favor of new publish-delay feature of katyo/publish-crates

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
