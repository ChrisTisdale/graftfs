# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.2.2 (2026-07-22)

### Chore

 - <csr-id-e36d6ef1da4e7fc841b911e7d95b6152b50f23b4/> Updating to the latest dependices
   Updating the dependices to the latest

### New Features

 - <csr-id-a8cbf3664958dba6155c8e807b9e12d01610aebb/> Updating for support for upcoming clap breaking changes
   Updating the code for upcoming breaking changes with clap v5.
   
   Fixed an issue where packages didn't handle the home directory
   correctly.
 - <csr-id-ce74d5a3a9812ca1c2268ad833d9d076ff0a09ab/> Updating dependencies and removing strip from xtask
   Updating dependencies to the latest versions.
   
   Removing strip from xtask it is now done on build

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 2 calendar days.
 - 3 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#23](https://github.com/ChrisTisdale/graftfs/issues/23)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#23](https://github.com/ChrisTisdale/graftfs/issues/23)**
    - Updating to the latest dependices ([`e36d6ef`](https://github.com/ChrisTisdale/graftfs/commit/e36d6ef1da4e7fc841b911e7d95b6152b50f23b4))
 * **Uncategorized**
    - Updating for support for upcoming clap breaking changes ([`a8cbf36`](https://github.com/ChrisTisdale/graftfs/commit/a8cbf3664958dba6155c8e807b9e12d01610aebb))
    - Updating dependencies and removing strip from xtask ([`ce74d5a`](https://github.com/ChrisTisdale/graftfs/commit/ce74d5a3a9812ca1c2268ad833d9d076ff0a09ab))
</details>

## v1.2.1 (2026-07-19)

### New Features

 - <csr-id-761b1a8347a5b1a2bd768d118b4433326126e788/> Updating dependencies
   Updating to the latest dependencies
 - <csr-id-d3e1575f8dad28f139b96f110bb4e65fde30a273/> Updating dependencies
   Updating to the latest dependencies
 - <csr-id-005806d5cd58903a86bdfcf390eee4acc0b9c864/> Support exporting the current configuration file
   This adds support to export the current configuration file or a user
   provided configuration file to either standard out or a file provided by
   the user.
 - <csr-id-c2932f50a92f27ef88beab43e70bce892169b848/> Updating dependencies
   Updating to the latest dependencies

### Bug Fixes

 - <csr-id-bc1b9f0055ad02c298b4946b8668f307fb9f36cb/> Addressing issues with help values
   Addressing issues with help values either being missing or incorrect.
   This will not correctly list the supported values for enum types.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 3 calendar days.
 - 6 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#22](https://github.com/ChrisTisdale/graftfs/issues/22)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#22](https://github.com/ChrisTisdale/graftfs/issues/22)**
    - Support exporting the current configuration file ([`005806d`](https://github.com/ChrisTisdale/graftfs/commit/005806d5cd58903a86bdfcf390eee4acc0b9c864))
 * **Uncategorized**
    - Release graftfs v1.2.1 ([`e54dc9b`](https://github.com/ChrisTisdale/graftfs/commit/e54dc9b313a6cef92fd166d49fcd5fed85903eb5))
    - Addressing issues with help values ([`bc1b9f0`](https://github.com/ChrisTisdale/graftfs/commit/bc1b9f0055ad02c298b4946b8668f307fb9f36cb))
    - Updating dependencies ([`761b1a8`](https://github.com/ChrisTisdale/graftfs/commit/761b1a8347a5b1a2bd768d118b4433326126e788))
    - Updating dependencies ([`d3e1575`](https://github.com/ChrisTisdale/graftfs/commit/d3e1575f8dad28f139b96f110bb4e65fde30a273))
    - Updating dependencies ([`c2932f5`](https://github.com/ChrisTisdale/graftfs/commit/c2932f50a92f27ef88beab43e70bce892169b848))
</details>

## v1.2.0 (2026-07-13)

### New Features

 - <csr-id-117432c9fd26021dc0defb27bf590938fa1a39aa/> Adding short symlinking support
   Adding support for shorted symlinking.  The default will be short
   symlinks but it can be changed either via the configuration file or via
   the commandline.

### Bug Fixes

 - <csr-id-4497b7855d64374194eb3eab178a1a2d4230ef24/> Fixing publish
   Addressing an issue with tests that where failing publishing
 - <csr-id-078e1d98d65bb02b034d2ac1a8c02e22f83d7059/> Fixing simulation not handling restow
   Addressed an issue with restow not printing that it was linking items.
   
   Addressing issues with simulation mode when stowing without folding
   directories
 - <csr-id-3c3489d4aa16234f280aef770077e7a7b9747ef9/> Addressing warnings
   Putting back disable warnings for enum names.  They match the shell names

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 5 calendar days.
 - 6 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#18](https://github.com/ChrisTisdale/graftfs/issues/18), [#20](https://github.com/ChrisTisdale/graftfs/issues/20)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#18](https://github.com/ChrisTisdale/graftfs/issues/18)**
    - Adding short symlinking support ([`117432c`](https://github.com/ChrisTisdale/graftfs/commit/117432c9fd26021dc0defb27bf590938fa1a39aa))
 * **[#20](https://github.com/ChrisTisdale/graftfs/issues/20)**
    - Fixing simulation not handling restow ([`078e1d9`](https://github.com/ChrisTisdale/graftfs/commit/078e1d98d65bb02b034d2ac1a8c02e22f83d7059))
 * **Uncategorized**
    - Release graftfs v1.2.0 ([`85203b2`](https://github.com/ChrisTisdale/graftfs/commit/85203b2f48b9635b4cfd43767327ed965dc4905a))
    - Fixing publish ([`4497b78`](https://github.com/ChrisTisdale/graftfs/commit/4497b7855d64374194eb3eab178a1a2d4230ef24))
    - Addressing warnings ([`3c3489d`](https://github.com/ChrisTisdale/graftfs/commit/3c3489d4aa16234f280aef770077e7a7b9747ef9))
</details>

## v1.1.0 (2026-07-07)

### Documentation

 - <csr-id-bec2096129a8e9b7764ee7b8e98a9640a93627e1/> Moving changelog to the package scope
   Moving all the previous releases into the package scope for changelogs

### New Features

 - <csr-id-097ef89587f5d25584f02a51c6d0a34e93450793/> Adding nushell completion support
   Adding support for nushell completions.  The completion generation can
   be use and enabled via a feature flag (nushell).
 - <csr-id-2961d15f76f0d4d030b5aabe36a78dadfb5ce722/> Adding exporting completions to a file
   Adding support for exporting completions to a file as an optional
   argument.
 - <csr-id-172539360413136742f93f03b132cd0c7493fdc4/> Adding better error display
   Moving error to snafu which will provide more detailed errors.  This
   will include additional details such as which file or parameter caused
   the issue.

### Bug Fixes

 - <csr-id-e05c5ba8a2b46ae8bd386367bb4aba6c58fa1927/> Fixing subcommand man manpages
   Fixing an issue where subcommands where not being rendered for man pages

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 10 calendar days.
 - 10 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#12](https://github.com/ChrisTisdale/graftfs/issues/12), [#15](https://github.com/ChrisTisdale/graftfs/issues/15), [#16](https://github.com/ChrisTisdale/graftfs/issues/16)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#12](https://github.com/ChrisTisdale/graftfs/issues/12)**
    - Adding better error display ([`1725393`](https://github.com/ChrisTisdale/graftfs/commit/172539360413136742f93f03b132cd0c7493fdc4))
 * **[#15](https://github.com/ChrisTisdale/graftfs/issues/15)**
    - Adding exporting completions to a file ([`2961d15`](https://github.com/ChrisTisdale/graftfs/commit/2961d15f76f0d4d030b5aabe36a78dadfb5ce722))
 * **[#16](https://github.com/ChrisTisdale/graftfs/issues/16)**
    - Adding nushell completion support ([`097ef89`](https://github.com/ChrisTisdale/graftfs/commit/097ef89587f5d25584f02a51c6d0a34e93450793))
 * **Uncategorized**
    - Release graftfs v1.1.0 ([`83d5e38`](https://github.com/ChrisTisdale/graftfs/commit/83d5e384f42729483dc89203dc541f95f61ccba4))
    - Fixing subcommand man manpages ([`e05c5ba`](https://github.com/ChrisTisdale/graftfs/commit/e05c5ba8a2b46ae8bd386367bb4aba6c58fa1927))
    - Moving changelog to the package scope ([`bec2096`](https://github.com/ChrisTisdale/graftfs/commit/bec2096129a8e9b7764ee7b8e98a9640a93627e1))
</details>

## v1.0.0 (2026-06-27)

### New Features

 - <csr-id-f1615a578608031af1587b167975f53723faba67/> Adding configurable config directory
   Adding the ability for configuring the applications config path.
 - <csr-id-0030f6c2562f676309cd32946aa3ec185c4690d4/> Allow for optional completions arguments
   Allow for the shell to be an optional argument.  When the shell isn't
   provided this will attempt to grab the active shell and use that.
 - <csr-id-d167cf68555941b9162920daebcffdfdcdb62604/> Adding man pages support
   Adding a new xtask for generating man pages.  This task can be run by
   anything that installs graft.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 5 calendar days.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#10](https://github.com/ChrisTisdale/graftfs/issues/10), [#6](https://github.com/ChrisTisdale/graftfs/issues/6), [#8](https://github.com/ChrisTisdale/graftfs/issues/8)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#10](https://github.com/ChrisTisdale/graftfs/issues/10)**
    - Adding configurable config directory ([`f1615a5`](https://github.com/ChrisTisdale/graftfs/commit/f1615a578608031af1587b167975f53723faba67))
 * **[#6](https://github.com/ChrisTisdale/graftfs/issues/6)**
    - Adding man pages support ([`d167cf6`](https://github.com/ChrisTisdale/graftfs/commit/d167cf68555941b9162920daebcffdfdcdb62604))
 * **[#8](https://github.com/ChrisTisdale/graftfs/issues/8)**
    - Allow for optional completions arguments ([`0030f6c`](https://github.com/ChrisTisdale/graftfs/commit/0030f6c2562f676309cd32946aa3ec185c4690d4))
 * **Uncategorized**
    - Release graftfs v1.0.0 ([`406e02a`](https://github.com/ChrisTisdale/graftfs/commit/406e02a373e8130849874aec5d6cce4581a29aea))
</details>

## v0.4.2 (2026-06-20)

### New Features

 - <csr-id-6e35b0cba3f37672fa633e7bce8e572f869eaacc/> Adding more OS support
   Adding macos to the builds for actions.
   
   Moving support for configs for mac and linux into just unix for support
   for other unix linux operating systems
 - <csr-id-38377925bcb4191949d08334df7fb3f334cb13d7/> Support multiple packages
   Allow for multiple packages to be stowed, listed, or unstowed in one
   operation.

### Bug Fixes

 - <csr-id-d1183b716d237e642e4f0246ab98115ce08b555f/> More package cleanup
   Fixing typo in exclude file list
 - <csr-id-e6742878b01ab2fdf70b13009ddfb7d95c50a206/> Package cleanup
   Removing files that are note needed when publishing

## v0.4.1 (2026-06-17)

### Bug Fixes

 - <csr-id-3266c11acc60d9c82d7313bad5c27dc6c88c1508/> Fixing no color support
   Fixing an issue where no color commnad line argument was getting ignored
   for writting to the console in simulator mode or list mode

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

## v0.3.1 (2026-06-14)

### Bug Fixes

 - <csr-id-4c3fde2c8f650397012318ad509dc2afa12e8eea/> Fixing matching issues
   Addressing issues where matches would return on a partial match instead
   of a full match.  This causes issues for things like gitconfig files

## v0.3.0 (2026-06-14)

### New Features

 - <csr-id-16d0969d5eee43ce3cef8464253ea7d318f01e45/> Moving completions to feature
   Moving the completions to a feature toggle as they are not fully
   supported and still experimental.
 - <csr-id-05cf65240884246638f574e656cb8caa4c60fb93/> Cleaning up completions
   Doing some code cleanup for command line completions
 - <csr-id-4c4f742ba8e8e3b2717f215b64c75a13064e16aa/> Shell completion support
   Adding support for generating command line completions for different shells

## v0.2.2 (2026-06-13)

<csr-id-dc71afbcdfc0f9266f7a517154b5408a94714e91/>
<csr-id-30ee6f559d6240abb10f8d858bab4f07ebc1c565/>

### Chore

 - <csr-id-dc71afbcdfc0f9266f7a517154b5408a94714e91/> Updating dependencies
   Updating the dependencies.
 - <csr-id-30ee6f559d6240abb10f8d858bab4f07ebc1c565/> Updating dependencies
   Updating the dependencies.

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

## v0.2.0 (2026-06-06)

### New Features

 - <csr-id-43af1330146879aa9709a2fc645fcc4d5ff4e3e9/> Moving windows to canonicalize paths.  Adding color support
   This moves the windows to canonicalize paths.  This will allow for long path support on windows.
   
   This adds color support and the ability to turn it off either via the
   command line or a configuration file.

## v0.1.0 (2026-06-05)

