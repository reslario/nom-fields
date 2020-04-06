//! This crate provides a single function-like macro that
//! removes some boilerplate from a common pattern when using
//! the parsing framework [nom](https://www.crates.io/crates/nom).
//!
//! With this macro, the following parser...
//! ```
//! nom::combinator::map(
//!     nom::sequence::tuple((
//!         some_parser,
//!         some_other_parser,
//!         a_third_parser
//!     )),
//!     |(some_field, some_other_field, a_third_field)| SomeStruct {
//!         some_field,
//!         some_other_field,
//!         a_third_field
//!     }
//! )(input)
//! ```
//! ...becomes this:
//! ```
//! nom_fields::fields!(SomeStruct:
//!     some_field = some_parser,
//!     some_other_field = some_other_parser,
//!     a_third_field = a_third_parser
//! )(input)
//! ```

/// The macro this crate provides. See the [top-level documentation](index.html) for details.
#[macro_export]
macro_rules! fields {
    ($strct:ident: $($field:ident = $parser:expr),+) => {
        nom::combinator::map(
            nom::sequence::tuple((
                $($parser),+
            )),
            |($($field),+)| $strct {
                $($field),+
            }
        )
    };
}
