## Work in progress

Serde support for [Extensible Data Notation (edn)](https://github.com/edn-format/edn), the S-expressions superset used as syntax for the [clojure](https://clojure.org/) programming language.  
At the moment it uses [edn.rs](https://docs.rs/edn/) to bootstrap development. The parser will be replaced in the future.

`serde_edn` also provides its own `Value` type and `edn!` macro for constructing values

`serde_edn` is heavily inspired by and modeled after [serde_json](https://crates.io/crates/serde_json)


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


## Unresolved questions

* how to map all edn constructs to rust types
* handling of symbols and keywords
* restrict deserialization to matching types ? (i.e. do we want to allow deserializing a Vec from an edn `#{...}` set ?)


## Working items list

* [ ] Serialization to string
* [ ] Serialization to Value
* [x] Deserialization from str (partial support)
* [x] Deserialization from Value (partial support)
* [x] `edn!` macro
* [x] utility functions for `Value`


## Limitations

* the `edn!` macro has some inherent limitations, due to the different treatment of whitespace and tokens between rust and edn syntax.


## Future goals

* fix or replace edn.rs to conform to spec
* bignum support
