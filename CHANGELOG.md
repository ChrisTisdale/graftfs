# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.2.1 (2026-06-12)

### Documentation

 - <csr-id-dfd63235e9392f093f0fb7ca1b51dfd9eb5d65d9/> Adding installation from crates IO
   This add additional instrunctions for installing and working with the
   package from crates IO

### Performance

 - <csr-id-68a8961313f0dd765a771cd8301cd649f5d0cabd/> Clone and Rc fixes
   This address places where clone is used or Rc was being used that added
   additional overhead and memory allocations that could be avoided
 - <csr-id-947a57a8f9b0a95992bd65c27c461da02a8ab6b0/> Updating to limiting cloning when not folding
   This updates to limit clone and additional allocations when a directory
   is not folded.
   
   Moving to include symbols in release for better debugging if a crash
   occurs.  This will increase the binary size but provides better
   diagnostics for release builds.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 1 calendar day.
 - 5 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Clone and Rc fixes ([`68a8961`](https://github.com/ChrisTisdale/graftfs/commit/68a8961313f0dd765a771cd8301cd649f5d0cabd))
    - Adding installation from crates IO ([`dfd6323`](https://github.com/ChrisTisdale/graftfs/commit/dfd63235e9392f093f0fb7ca1b51dfd9eb5d65d9))
    - Updating to limiting cloning when not folding ([`947a57a`](https://github.com/ChrisTisdale/graftfs/commit/947a57a8f9b0a95992bd65c27c461da02a8ab6b0))
</details>

## v0.2.0 (2026-06-06)

### New Features

 - <csr-id-43af1330146879aa9709a2fc645fcc4d5ff4e3e9/> Moving windows to canonicalize paths.  Adding color support
   This moves the windows to canonicalize paths.  This will allow for long path support on windows.
   
   This adds color support and the ability to turn it off either via the
   command line or a configuration file.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release graftfs v0.2.0 ([`f82138f`](https://github.com/ChrisTisdale/graftfs/commit/f82138f609220af9e41e979cd904a4f92a25465e))
    - Moving windows to canonicalize paths.  Adding color support ([`43af133`](https://github.com/ChrisTisdale/graftfs/commit/43af1330146879aa9709a2fc645fcc4d5ff4e3e9))
</details>

## v0.1.0 (2026-06-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release graftfs v0.1.0 ([`f21bcd6`](https://github.com/ChrisTisdale/graftfs/commit/f21bcd6c45eedef377ba84a2e5026311dcc2c197))
    - Initial commit ([`85deab6`](https://github.com/ChrisTisdale/graftfs/commit/85deab6290c8910c23caf1ebf8b5420a8dd0c4c8))
</details>

