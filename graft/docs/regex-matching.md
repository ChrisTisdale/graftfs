# Regex matching

`src/commands/regex_matcher.rs`, `src/commands/regex_captures.rs`

Ignore and override patterns are regexes. Two independent axes control how they are evaluated, both
configured in `StowConfig` — see [`configuration.md`](configuration.md).

## `RegexStrategy` — which engine

`RegexMatcher` is an enum over `grep::regex::RegexMatcher` and `grep::pcre2::RegexMatcher`, exposing a
single `impl grep::matcher::Matcher`. `RegexCaptures` is the parallel enum for capture groups, and
`MatchError` unifies the two error types.

Adding a method to the `Matcher` impl means delegating in both arms. The `pcre2` support comes from the
`grep` crate's `pcre2` feature, enabled in `Cargo.toml`.

## `MatchingStrategy` — how many matchers

- **`combined`** (default) — `try_create_combined_matcher` compiles every pattern into one matcher via
  `build_many`. Faster; one pattern failing to compile loses the whole set.
- **`individual`** — `try_create_matcher` compiles one matcher per pattern.

## Compilation failures are warnings, not errors

Both constructors return `Option<Self>` and log via `warn!` on failure:

```rust
.map_err(|e| warn!("Failed to create Rust regex matcher for {item}: {e}"))
.ok()
```

A malformed pattern in a `.graft-ignore` file degrades matching rather than aborting the run. Preserve this
— turning a bad user pattern into a hard error is a behavior change, and note the asymmetry it creates with
`combined` mode, where one bad pattern silently discards all the others.
