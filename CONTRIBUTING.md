# Contributing to Atom

The following is a set of guidelines for contributing to fatigue and its packages. These are mostly guidelines, not rules. Use your best judgment, and feel free to propose changes to this document in a pull request.

#### Table Of Contents

[Code of Conduct](#code-of-conduct)

[How Can I Contribute?](#how-can-i-contribute)
* [Reporting Bugs](#reporting-bugs)
* [Suggesting Enhancements](#suggesting-enhancements)
* [Your First Code Contribution](#your-first-code-contribution)
* [Pull Requests](#pull-requests)

[Styleguides](#styleguides)
* [Git Commit Messages](#git-commit-messages)
* [JavaScript Styleguide](#javascript-styleguide)
* [CoffeeScript Styleguide](#coffeescript-styleguide)
* [Specs Styleguide](#specs-styleguide)
* [Documentation Styleguide](#documentation-styleguide)

[Additional Notes](#additional-notes)
* [Issue and Pull Request Labels](#issue-and-pull-request-labels)

## Code of Conduct

This project and everyone participating in it is governed by the [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues as you might find out that there is an existing issue for the bug.  Fill out [the required template](https://github.com/pdylanross/.github/blob/master/.github/ISSUE_TEMPLATE/bug_report.md), the information it asks for helps us resolve issues faster.

> **Note:** If you find a **Closed** issue that seems like it is the same thing that you're experiencing, open a new issue and include a link to the original issue in the body of your new one.

### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for fatigue, including completely new features and minor improvements to existing functionality. Following these guidelines helps maintainers and the community understand your suggestion and find related suggestions. Fill in [the template](https://github.com/pdylanross/.github/blob/master/.github/ISSUE_TEMPLATE/feature_request.md), including the steps that you imagine you would take if the feature you're requesting existed.

### Your First Code Contribution

Unsure where to begin contributing to Atom? You can start by looking through these `beginner` and `help-wanted` issues:

* [Beginner issues][beginner] - issues which should only require a few lines of code, and a test or two.
* [Help wanted issues][help-wanted] - issues which should be a bit more involved than `beginner` issues.

#### Local development

Included in the examples directory is a great test api to test fatigue with. The example api can be started by simply running `cargo run` from the test api directory. Feel free to add any functionality that may be needed to test your features to the test api. Linting and normal code quality rules do not apply to that project, but please, don't use it like a garbage bin. 

### Pull Requests

Please follow these steps to have your contribution considered by the maintainers:

1. Create or reference an existing issue for your pull request.
2. Follow the [styleguides](#styleguides)
3. After you submit your pull request, verify that all [status checks](https://help.github.com/articles/about-status-checks/) are passing <details><summary>What if the status checks are failing?</summary>If a status check is failing, and you believe that the failure is unrelated to your change, please leave a comment on the pull request explaining why you believe the failure is unrelated. A maintainer will re-run the status check for you. If we conclude that the failure was a false positive, then we will open an issue to track that problem with our status check suite.</details>

While the prerequisites above must be satisfied prior to having your pull request reviewed, the reviewer(s) may ask you to complete additional design work, tests, or other changes before your pull request can be ultimately accepted.

## Styleguides

### Git Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull requests liberally after the first line
* Use conventional commits:
  * break: the change contains breaking changes to the command line interface or configuration file format. (the library is nowhere near being any kind of stable, and as such we aren't even going to worry about that for now.)
  * feat: the change adds a new feature
  * fix: the change fixes an existing feature
  * doc: the change only updates documentation
  * ci: the change updates ci/cd
  * chore: dependency updates, or anything else not related to actual code

### Rust Stylguide

* No clippy warnings
* No build warnings
* Use cargo-sort to keep crates tidy


[beginner]:https://github.com/pdylanross/fatigue/issues?q=is%3Aopen+label%3A%22good+first+issue%22+sort%3Areactions-%2B1-desc
[help-wanted]:https://github.com/pdylanross/fatigue/issues?q=is%3Aopen+sort%3Areactions-%2B1-desc+label%3A%22help+wanted%22+
