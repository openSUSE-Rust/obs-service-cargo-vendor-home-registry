# OBS Service Cargo Vendor Home Registry

> [!IMPORTANT]
> This project's logic and code has now been merged to
> [obs-service-cargo](https://github.com/openSUSE-Rust/obs-service-cargo)
> as a new **method** for vendoring.
>
> No further updates will be added here in the future and thus, it will be
> archived.

[![CI](https://github.com/openSUSE-Rust/obs-service-cargo-vendor-home-registry/actions/workflows/ci.yml/badge.svg)](https://github.com/openSUSE-Rust/obs-service-cargo-vendor-home-registry/actions/workflows/ci.yml)
[![build result](https://build.opensuse.org/projects/devel:languages:rust/packages/obs-service-cargo-vendor-home-registry/badge.svg?type=percent)](https://build.opensuse.org/package/show/devel:languages:rust/obs-service-cargo-vendor-home-registry)

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
├── cargo_vendor_home_registry.service
├── CHANGELOG
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── CONTRIBUTORS.md
├── justfile
├── LICENCE
├── README.md
├── rustfmt.toml
└── src
    ├── audit.rs
    ├── cli.rs
    ├── lib.rs
    ├── main.rs
    └── opts.rs

2 directories, 16 files
```

# Features
- Allow custom root directory of the project if in case the "dumb" detection
fails.
- Allow option to set no root manifest. Packages such as `s390-tools` do
not have a root manifest as it is a **monorepo**. It's not even a [cargo
workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).
However, users must manually add the path to the extra manifests if needed
relative to project top-level directory.
- Tag support. Like `obs-service-cargo`, we support tags so you can have
multiple registry contexts.  However, that I still yet to find any.
- No need to set `respect-lockfiles`. Lockfiles are regenerated and shipped
in the tarball. Therefore, updated lockfiles are also shipped without worrying
if you have mismatching dependencies.

# How to use in a specfile

A typical specfile looks like this

```

%prep
%autosetup -a1

%build
export CARGO_HOME=$PWD/.cargo
%{cargo_build}

%install
export CARGO_HOME=$PWD/.cargo
%{cargo_install}

%check
export CARGO_HOME=$PWD/.cargo
%{cargo_test}


```


# Q&A

For questions and answers, you can head over to our
[Discussion](https://github.com/orgs/openSUSE-Rust/discussions) page on GitHub.
