//! nexus-serde — serde Serializer + Deserializer for the
//! [nexus](https://github.com/LiGoldragon/nexus) messaging protocol.
//!
//! nexus is a strict superset of [nota](https://github.com/LiGoldragon/nota).
//! nexus-serde round-trips every nota value identically, and additionally
//! handles the three query-layer wrapper types:
//!
//! - [`Bind`] — a `@`-prefixed bind hole. `Bind("h".into())` → `@h`.
//! - [`Mutate<T>`] — `~`-prefixed mutation marker. `Mutate(x)` → `~<x>`.
//! - [`Negate<T>`] — `!`-prefixed negation. `Negate(x)` → `!<x>`.
//!
//! Pattern / Constrain / Shape containers (`(| |)`, `{| |}`, `{ }`) are
//! recognised by the lexer but not yet mapped to wrapper types — their
//! Rust-type design is deferred to the consumer crates (nexusd,
//! nexus-cli) which will define message types against the grammar.
//!
//! ```
//! use nexus_serde::{Bind, Mutate, Negate};
//!
//! #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
//! struct Point { horizontal: f64, vertical: f64 }
//!
//! // Plain assertion — identical to nota.
//! let p = Point { horizontal: 3.0, vertical: 4.0 };
//! assert_eq!(
//!     nexus_serde::to_string(&p)?,
//!     "(Point horizontal=3.0 vertical=4.0)"
//! );
//!
//! // Mutation marker.
//! assert_eq!(
//!     nexus_serde::to_string(&Mutate(p))?,
//!     "~(Point horizontal=3.0 vertical=4.0)"
//! );
//!
//! // Negation.
//! #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
//! enum Status { Active }
//! assert_eq!(nexus_serde::to_string(&Negate(Status::Active))?, "!Active");
//!
//! // Bind hole.
//! let b = Bind("h".into());
//! assert_eq!(nexus_serde::to_string(&b)?, "@h");
//! let back: Bind = nexus_serde::from_str("@h")?;
//! assert_eq!(back, b);
//! # Ok::<(), nexus_serde::Error>(())
//! ```

mod de;
mod error;
mod lexer;
mod ser;

pub use de::{from_str, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_string, Serializer};

use serde::{Deserialize, Serialize};

/// A `@`-prefixed bind hole — a named slot the reader fills during a
/// pattern match. Holds an identifier-shaped string (alphanumeric, `-`,
/// `_`).
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "@NexusBind")]
pub struct Bind(pub String);

/// Marks a value as a mutation — `~value`. Asserting a `Mutate<T>`
/// means "replace / overwrite the prior identity of this value."
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "@NexusMutate")]
pub struct Mutate<T>(pub T);

/// Marks a value or pattern as negated — `!value`. Asserting a
/// `Negate<T>` means "this is not true / retract this fact."
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "@NexusNegate")]
pub struct Negate<T>(pub T);
