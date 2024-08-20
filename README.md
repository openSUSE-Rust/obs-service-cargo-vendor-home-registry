# OBS Service Cargo Vendor Home Registry

A rewrite of the [OBS Service Cargo Vendor](https://github.com/Firstyear/obs-service-cargo/).

The goals of this project are
- reduce boilerplate code by more than 50%
- use more idiomatic Rust code
- handle error messages better

## Why rewrite?

`cargo vendor` is sus so I decided to go with vendoring the cargo home registry `$CARGO_HOME` instead 😳
