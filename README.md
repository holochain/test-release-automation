# test-release-automation

## How to Try This Out

This repository demonstrates a release automation workflow using [`release-please`](https://github.com/googleapis/release-please) and [`cargo-workspaces`](https://github.com/pksunkara/cargo-workspaces), tailored for multi-crate Rust workspaces and multi-branch release flows (e.g., `main`, `main-0.1`).

### Overview

- `release-please` opens release PRs per crate, based on commit history.
- Merging a release PR triggers a version bump and creates a Git tag (e.g. `release-v0.4.1`).
- A separate `Publish Crates` workflow listens for these tags and publishes updated crates to [crates.io](https://crates.io) via `cargo workspaces`.

### Triggering a Release PR

Release PRs are automatically created by the `release-please-action` workflow whenever you push to a configured branch (e.g., `main`, `main-0.1`), and there's at least one commit following conventional commit guidelines.

#### Examples

**Patch Release** (e.g. bugfix)

```bash
git checkout main
echo "// fix: handle edge case" >> crates/core/src/lib.rs
git add .
git commit -m "fix(core): handle edge case in parser"
git push
```

**Minor Release** (e.g. new feature)

```bash
git checkout main
echo "// feat: add feature" >> crates/web/src/lib.rs
git add .
git commit -m "feat(web): add new config flag"
git push
```

**Major Release** (e.g. breaking change)

```bash
git checkout main
echo "// BREAKING CHANGE: renamed module" >> crates/cli/src/main.rs
git add .
git commit -m "refactor(cli): rename foo module\n\nBREAKING CHANGE: foo module renamed to bar"
git push
```

### Working with Maintenance Branches

You can also simulate patch releases from older versions using branches like main-0.1.

#### Checkout to release branch

```bash
git checkout main-0.1
```

#### Make a patch change

```bash
echo "// fix: maintenance patch" >> crates/core/src/lib.rs
git add .
git commit -m "fix(core): fix legacy behavior"
git push
```

The release-please action on main-0.1 will generate a scoped release PR and tag (e.g., release-v0.1.4), which will then be used for publishing.

### Merging the Release PR

After a release PR is opened:

- Review the changelog, version bumps, and affected crates.
- Merge the PR. This will push a release-v\* tag (e.g., release-v0.1.4) to the repo and respective tags for each workspace crate that should be updated.

### Publishing Crates

Once a tag matching release-v\* is pushed, the Publish Crates workflow will:

- Install `cargo-workspaces`
- Detect changed crates via Git tags
- Publish all version-bumped crates to crates.io in the right order
