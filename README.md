[![](http://meritbadge.herokuapp.com/nom-fields)](https://crates.io/crates/nom-fields)
[![Docs](https://docs.rs/nom-fields/badge.svg)](https://docs.rs/nom-fields/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)]()

# nom-fields
This crate provides a single function-like macro that
removes some boilerplate from a common pattern when using
the parsing framework [nom](https://www.crates.io/crates/nom).

With this macro, the following parser...
```rust
nom::combinator::map(
    nom::sequence::tuple((
        some_parser,
        some_other_parser,
        a_third_parser
    )),
    |(some_field, some_other_field, a_third_field)| SomeStruct {
        some_field,
        some_other_field,
        a_third_field
    }
)(input)
```
...becomes this:
```rust
nom_fields::fields!(SomeStruct:
    some_field = some_parser,
    some_other_field = some_other_parser,
    a_third_field = a_third_parser
)(input)
```
