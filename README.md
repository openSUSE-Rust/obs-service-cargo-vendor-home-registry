# OBS Service Cargo Vendor Home Registry

A rewrite of the [OBS Service Cargo Vendor](https://github.com/Firstyear/obs-service-cargo/).

The goals of this project are
- reduce boilerplate code by more than 50%
- use more idiomatic Rust code
- handle error messages better

## Why rewrite?

`cargo vendor` is sus so I decided to go with vendoring the cargo home registry `$CARGO_HOME` instead 😳

# Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── CHANGELOG.md
├── cliff.toml
├── LICENCE
├── README.md
└── src
    ├── cli.rs
    ├── compress.rs
    ├── decompress.rs
    ├── lib.rs
    ├── main.rs
    └── opts.rs

2 directories, 12 files
```

## Features

Introducing *manifest_options*. This option allows users to finely-tune
cargo options for a specified manifest/crate path. For example, if user
wants to update crate A, they can do so with `--manifest-options crateA,true`
but they want to leave B alone so `--manifest-options crateB,false`.

> [!NOTE]
> This is pretty handy if users want to set each crate manually. The default
update option for all crates though is true. One can set a flag to disable
update using `--lockfile-all` flag for all crates.

### About `respect-lockfile`

*respect-lockfile* is now part of *manifest_options*. How to use it?

It will be just a simple third option in the *manifest_option* i.e. `--manifest-options cratePath,false,true`

*respect-lockfile* will always supersede *update* option if set to `true`.

When a crate does not have a lockfile at all. We will be warning the users
that if such a case arises, we will force generate the lockfile using `cargo
update` with the following scenarios
- if network connection exists, attempt to just run `cargo update` normally;
otherwise
- add the `--offline` option

If the lockfile exists or regenerated (because it didn't exist at first),
we will add a `--locked` option when attempting to `cargo fetch`.

## Workspace crates

## Working with multiple unrelated crates

