## Work in progress

Serde support for [edn](https://github.com/edn-format/edn).
At the moment it uses [edn.rs](https://docs.rs/edn/) to bootstrap development. The parser will be replaced in the future.

`serde_edn` also provides its own `Value` type and `edn!` macro for constructing values

`serde_edn` is heavily inspired and modeled after [serde_json](https://crates.io/crates/serde_json)


## Working assumptions

Serialization:

| from Rust | to edn |
| ---  | --- |
| tuple, tuple struct | list (or vector ?) |
| struct, struct variant | map keyed with keywords |
| enum | map? |
| unit variant | keyword |
| Vec | vector |
| HashMap, BTreeMap | map |
| HashSet, BTreeSet | set |
| LinkedList | list |


## Unresolved question

* how to map all edn constructs to rust types
* handling of symbols and keywords


## TODO list

* [ ] Serialization
* [ ] Deserialization
* [ ] `edn!` macro
* [x] utility functions for `Value`


## Limitations

* the `edn!` macro has some inherent limitations, due to the different treatment of whitespace and tokens between rust and edn syntax.


## Future goals

* fix or replace edn.rs to conform to spec
* bignum support
