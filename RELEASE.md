# Release Process

This page describes how to release components in this repository.

## Checklist Template

- [ ] Make sure we are feature complete
- [ ] Check links
- [ ] Update RUST versions
- [ ] Perform testing
- [ ] Create local release tag
- [ ] Create github tag
- [ ] Create github pre-release of kuksa-rust-sdk
- [ ] Merge release-branches (if used) back to master/main
- [ ] Project lead informed
- [ ] Official release approval received
- [ ] Releases transformed to official releases

## Release Process in short

1. Merge all functional changes that should be included
2. Perform testing according to https://github.com/eclipse/kuksa.val/wiki/Release-Testing
3. When all findings are corrected and if needed retested, then tag and upload a tag
4. If it works create a Github pre-release
5. Ask project owners to review and approve release, When done transform to real release

## Checking links

The commands below can be used to verify that links in markdown files works as expected

`npm install -g markdown-link-check`

`find . -name \*.md -print0 | xargs -0 -n1 markdown-link-check`

`markdown-link-check` may give some false positives, they can be suppressed by accepting certain error codes (in addition to 200). A possible example is below:

`markdown-link-check -q -a 200,404 *.md`

To check all but ignore some error codes do from root repo folder:

`find . -name \*.md  -print0  | xargs -0 -n1 markdown-link-check -q -a 403,200,404`

As of today CI ignores 200 and 404, and due to limitations in markdown-link-check it also ignores some links where false positive 403 errors are otherwisegiven. It is anyway trecommended to run manually before release including 403 and analyze what needs to be changed.

## Update Rust versions

Find every `Cargo.toml`, replace version. Do not (by default) update edition, it concerns Rust edition, not package release date.

Can be done by running `scripts/prepare_release.sh <version>`

Do a cargo update, make sure that version is as expected

```shell
erik@debian3:~/kuksa-databroker$ !2247
cargo update
Updating crates.io index
Updating databroker v0.3.1 (/home/erik/kuksa.val/kuksa_databroker/databroker) -> v0.4.0
Updating databroker-cli v0.3.1 (/home/erik/kuksa.val/kuksa_databroker/databroker-cli) -> v0.4.0
Updating databroker-examples v0.3.1 (/home/erik/kuksa.val/kuksa_databroker/databroker-examples) -> v0.4.0
Updating databroker-proto v0.3.1 (/home/erik/kuksa.val/kuksa_databroker/databroker-proto) -> v0.4.0
```

## Proposed Versioning Scheme

Use `X.Y.Z` only for the commit that matches the release. For other use-cases use a pre-release identifier.
- For development branches set version to `X.Y.Z-pre.0`, for example `0.4.7-dev.0` where the number is a possible number for the next release on that branch. In general it does not matter that much if you after release of 0.4.6 sets it to 0.4.7-dev.0 or 0.5.0-dev.0, the important part is to highlight that this is a development version. You should never create a tag or release with a "dev" version
- If you need to tag and/or release something for testing purposes, use alpha or rc as prerelease-tag

References:
- https://semver.org/
- https://users.rust-lang.org/t/alpha-beta-rc-with-cargo-semantic-versioning/69799

## Create Release

- Go here: https://github.com/eclipse-kuksa/kuksa-rust-sdk/releases/new
- Choose a tag or create a new one, e.g. `0.6.0`
- Choose a branch: `main` (or the one corresponding to the release)
- Add a new release title. e.g. `KUKSA Rust SDK 0.6.0`
- Press on "Generate release notes"
- Select "Set as a pre-release"
- Copy instructions based on previous release (part above ## What's Changed), i.e. information on how to fetch from docker
- Also say something about the included files
- Select "Set as a pre-release"

## Create a Release & maintenance branches (If not already done)

- Create `release/X.Y.Z`, e.g. `release/0.5.0`. This is if anyone prefers a branch, possibly together with a tag, to reference this particular release. Seems like Yocto/AGL benefits from this
- Create a patch branch of the form `0.5.X`, this is for patches, if any

## Inform Project Lead

It is the official project lead that needs to approve that the release is marked as an official release and as latest release

## Publish to crates.io

Once the release has been marked as an official release (not a pre-release) it should automatically be pushed to crates.io.

## Update main to point to new version

Update to something like 0.6.0-dev.0 Can be done by running `scripts/prepare_release.sh <version>`

## Checklist Template

- [ ] Make sure we are feature complete
- [ ] Check links
- [ ] Update RUST versions
- [ ] Perform testing
- [ ] Create local release tag
- [ ] Create github tag
- [ ] Create github pre-release of kuksa-rust-sdk
- [ ] Merge release-branches (if used) back to master/main
- [ ] Project lead informed
- [ ] Official release approval received
- [ ] Releases transformed to official releases
