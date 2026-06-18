# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.4.1 (2026-06-17)

### Bug Fixes

 - <csr-id-3266c11acc60d9c82d7313bad5c27dc6c88c1508/> Fixing no color support
   Fixing an issue where no color commnad line argument was getting ignored
   for writting to the console in simulator mode or list mode

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fixing no color support ([`3266c11`](https://github.com/ChrisTisdale/graftfs/commit/3266c11acc60d9c82d7313bad5c27dc6c88c1508))
</details>

## v0.4.0 (2026-06-17)

### New Features

 - <csr-id-d874e5c2a777ac745b2fd8fd625a7d0ced6ee663/> Cleaning up command line args
   Moving source to a required argument via clap
 - <csr-id-18810c50bac728163efebe28e0c93e3f9b557fe2/> Enabling fulling completions support

### Bug Fixes

 - <csr-id-96f53854d332df4bfcae1ee58ee4b7b8eb41a471/> Fixing help docs
   Correcting issues with the help stating incorrect defaults

### New Features (BREAKING)

 - <csr-id-982275f8fb4d960f753881d888795252437f22a1/> Reworking commands to limit required arguments

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 3 calendar days.
 - 3 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release graftfs v0.4.0 ([`ecfb95f`](https://github.com/ChrisTisdale/graftfs/commit/ecfb95f8a78ebf4fdea3ca23e09fc15b14822090))
    - Fixing help docs ([`96f5385`](https://github.com/ChrisTisdale/graftfs/commit/96f53854d332df4bfcae1ee58ee4b7b8eb41a471))
    - Cleaning up command line args ([`d874e5c`](https://github.com/ChrisTisdale/graftfs/commit/d874e5c2a777ac745b2fd8fd625a7d0ced6ee663))
    - Reworking commands to limit required arguments ([`982275f`](https://github.com/ChrisTisdale/graftfs/commit/982275f8fb4d960f753881d888795252437f22a1))
    - Enabling fulling completions support ([`18810c5`](https://github.com/ChrisTisdale/graftfs/commit/18810c50bac728163efebe28e0c93e3f9b557fe2))
</details>

## v0.3.1 (2026-06-14)

### Bug Fixes

 - <csr-id-4c3fde2c8f650397012318ad509dc2afa12e8eea/> Fixing matching issues
   Addressing issues where matches would return on a partial match instead
   of a full match.  This causes issues for things like gitconfig files

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release graftfs v0.3.1 ([`bbb6389`](https://github.com/ChrisTisdale/graftfs/commit/bbb6389b27869bd96f0df359f634f962a8430b85))
    - Fixing matching issues ([`4c3fde2`](https://github.com/ChrisTisdale/graftfs/commit/4c3fde2c8f650397012318ad509dc2afa12e8eea))
</details>

## v0.3.0 (2026-06-14)

### New Features

 - <csr-id-16d0969d5eee43ce3cef8464253ea7d318f01e45/> Moving completions to feature
   Moving the completions to a feature toggle as they are not fully
   supported and still experimental.
 - <csr-id-05cf65240884246638f574e656cb8caa4c60fb93/> Cleaning up completions
   Doing some code cleanup for command line completions
 - <csr-id-4c4f742ba8e8e3b2717f215b64c75a13064e16aa/> Shell completion support
   Adding support for generating command line completions for different shells

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release graftfs v0.3.0 ([`84a44f5`](https://github.com/ChrisTisdale/graftfs/commit/84a44f55d7bcd07dc55faff00eb1ab90f2f928b6))
    - Moving completions to feature ([`16d0969`](https://github.com/ChrisTisdale/graftfs/commit/16d0969d5eee43ce3cef8464253ea7d318f01e45))
    - Cleaning up completions ([`05cf652`](https://github.com/ChrisTisdale/graftfs/commit/05cf65240884246638f574e656cb8caa4c60fb93))
    - Shell completion support ([`4c4f742`](https://github.com/ChrisTisdale/graftfs/commit/4c4f742ba8e8e3b2717f215b64c75a13064e16aa))
</details>

## v0.2.2 (2026-06-13)

<csr-id-dc71afbcdfc0f9266f7a517154b5408a94714e91/>
<csr-id-30ee6f559d6240abb10f8d858bab4f07ebc1c565/>

### Chore

 - <csr-id-dc71afbcdfc0f9266f7a517154b5408a94714e91/> Updating dependencies
   Updating the dependencies.
 - <csr-id-30ee6f559d6240abb10f8d858bab4f07ebc1c565/> Updating dependencies
   Updating the dependencies.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 1 day passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release graftfs v0.2.2 ([`eeb12a1`](https://github.com/ChrisTisdale/graftfs/commit/eeb12a1e9bacb80c78f559141a3ed125d095f46f))
    - Updating dependencies ([`dc71afb`](https://github.com/ChrisTisdale/graftfs/commit/dc71afbcdfc0f9266f7a517154b5408a94714e91))
    - Updating dependencies ([`30ee6f5`](https://github.com/ChrisTisdale/graftfs/commit/30ee6f559d6240abb10f8d858bab4f07ebc1c565))
</details>

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

 - 4 commits contributed to the release over the course of 1 calendar day.
 - 5 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release graftfs v0.2.1 ([`d3c3cbe`](https://github.com/ChrisTisdale/graftfs/commit/d3c3cbe79396e2b795f823f7668b1d2a71ebe772))
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

