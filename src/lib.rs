//! nexus-serde — serde Serializer + Deserializer for the
//! [nexus](https://github.com/LiGoldragon/nexus) messaging protocol.
//!
//! Thin public façade over [`nota_serde_core`] in the Nexus dialect.
//! The kernel (Lexer, Token, Error, Serializer, Deserializer, and
//! the ser/de machinery) lives in that crate and is shared with
//! [nota-serde](https://github.com/LiGoldragon/nota-serde), which
//! uses the Nota dialect of the same kernel.
//!
//! nexus is a strict superset of [nota](https://github.com/LiGoldragon/nota).
//! Round-trips every nota value identically and additionally handles
//! three query-layer wrapper types:
//!
//! - [`Bind`] — a `@`-prefixed bind hole. `Bind("h".into())` → `@h`.
//! - [`Mutate<T>`] — `~`-prefixed mutation marker. `Mutate(x)` → `~<x>`.
//! - [`Negate<T>`] — `!`-prefixed negation. `Negate(x)` → `!<x>`.
//!
//! Pattern / Constrain / Shape containers (`(| |)`, `{| |}`, `{ }`) —
//! and the Tier-1 additions from
//! [mentci reports/013](https://github.com/LiGoldragon/mentci/blob/main/reports/013-nexus-syntax-proposal.md)
//! (`<| |>` stream, `(|| ||)` optional pattern, `{|| ||}` atomic txn)
//! — are recognised by the lexer but not yet mapped to wrapper
//! types. Their Rust-type design is deferred to the consumer crates
//! (nexusd, nexus-cli) which will define message types against the
//! grammar.
//!
//! ```
//! use nexus_serde::{Bind, Mutate, Negate};
//!
//! #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
//! struct Point { horizontal: f64, vertical: f64 }
//!
//! let p = Point { horizontal: 3.0, vertical: 4.0 };
//! assert_eq!(nexus_serde::to_string(&p)?, "(Point 3.0 4.0)");
//!
//! assert_eq!(nexus_serde::to_string(&Mutate(p))?, "~(Point 3.0 4.0)");
//!
//! #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
//! enum Status { Active }
//! assert_eq!(nexus_serde::to_string(&Negate(Status::Active))?, "!Active");
//!
//! let b = Bind("h".into());
//! assert_eq!(nexus_serde::to_string(&b)?, "@h");
//! let back: Bind = nexus_serde::from_str("@h")?;
//! assert_eq!(back, b);
//! # Ok::<(), nexus_serde::Error>(())
//! ```

pub use nota_serde_core::{
    from_str_nexus as from_str,
    to_string_nexus as to_string,
    Error, Result,
};

use serde::{Deserialize, Serialize};

/// A `@`-prefixed bind hole — a named slot the reader fills during a
/// pattern match. Holds an identifier-shaped string (camelCase or
/// kebab-case: first char `[a-z_]`, body `[a-z0-9_-]`).
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
