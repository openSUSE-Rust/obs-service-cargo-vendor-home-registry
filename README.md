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
├── justfile
├── LICENCE
├── README.md
├── rustfmt.toml
├── src
│   ├── audit.rs
│   ├── cli.rs
│   ├── lib.rs
│   ├── main.rs
│   └── opts.rs
└── vendor.toml

2 directories, 14 files
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
