# objid

[![Crates.io](https://img.shields.io/crates/v/objid.svg)](https://crates.io/crates/objid) [![Ubuntu](https://github.com/montao/objid/actions/workflows/rust.yml/badge.svg)](https://github.com/montao/objid/actions/workflows/rust.yml)

[![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/3877/badge)](https://bestpractices.coreinfrastructure.org/projects/3877)

The random Rust object identifier.

This program generates a random id suitable for code generators. 

* [Installation](#installation)
* [Commands](#commands)

## Installation

You can install `objid` from either this repository, or from Crates (once it's published):

```shell
# install from Cargo
$ cargo install objid

# install the latest from GitHub
$ cargo install --git https://github.com/montao/objid.git
```

## Commands

Running the program generates a new object identifier like the following. 

```shell
$ objid
Random object id: _e4lzU1OgCT
```
