# Shells and completions

`src/shell.rs`, `src/shell_converter_error.rs`

`Shell` is a local enum mirroring `clap_complete::Shell`. It exists rather than using `clap_complete::Shell`
directly so that the optional Nushell variant can be added behind `#[cfg(feature = "nushell")]`, which the
upstream enum cannot express.

```rust
pub enum Shell {
    Bash, Fish, Elvish, PowerShell, Zsh,
    #[cfg(feature = "nushell")] Nushell,
}
```

`TryFrom<clap_complete::Shell>` is fallible because upstream has variants this enum does not model; the
catch-all arm returns `ShellConverterError`. `Shell::from_env()` uses it to detect the caller's shell.

`impl Generator for Shell` delegates each of `file_name`, `generate`, and `try_generate` to the
corresponding `clap_complete::Shell` variant, or to `clap_complete_nushell::Nushell`.

## Adding a shell

A new variant must be added in six places in `src/shell.rs`:

1. The enum itself
2. `ValueEnum::value_variants`
3. `ValueEnum::to_possible_value` (aliases go here — `powershell` has `pwsh`, `nushell` has `nu`)
4. `Display`
5. `TryFrom<clap_complete::Shell>`, if upstream has a matching variant
6. All three `Generator` methods

Then mirror it in `xtask` — see [`../../xtask/AGENTS.md`](../../xtask/AGENTS.md). The man-page generator
keeps its own copies of this enum, and they are not checked against this one by the compiler. A variant
added here but not there produces man pages that omit the shell.

Missing any of the six is a silent failure: `cargo build` succeeds and the shell is simply unavailable or
renders wrong.
