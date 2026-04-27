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
//! six query-layer wrapper types:
//!
//! - [`Bind`] — a `@`-prefixed bind hole. `Bind("h".into())` → `@h`.
//! - [`Mutate<T>`] — `~`-prefixed mutation marker. `Mutate(record)` → `~(record …)`.
//! - [`Negate<T>`] — `!`-prefixed retraction. `Negate(record)` → `!(record …)`.
//! - [`Validate<T>`] — `?`-prefixed dry-run. `Validate(record)` → `?(record …)`.
//! - [`Subscribe<T>`] — `*`-prefixed continuous query. `Subscribe(pattern)` → `*(…)`.
//! - [`AtomicBatch<T>`] — `[| |]`-wrapped all-or-nothing edit list.
//!
//! Pattern / Constrain / Shape containers (`(| |)`, `{| |}`, `{ }`)
//! are recognised by the lexer but not mapped to serde wrapper
//! types — patterns are parsed by the nexus daemon's
//! [`QueryParser`](https://github.com/LiGoldragon/nexus/blob/main/src/parse.rs)
//! directly into typed `signal::QueryOp` values, bypassing serde's
//! enum-by-name dispatch.
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

/// Marks a verb as dry-run — `?value`. The daemon evaluates the
/// request and returns the would-be result without committing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "@NexusValidate")]
pub struct Validate<T>(pub T);

/// Marks a pattern as a continuous subscription — `*pattern`.
/// The first reply is a snapshot; further matches stream as events.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "@NexusSubscribe")]
pub struct Subscribe<T>(pub T);

/// Wraps a sequence of edit operations that must apply atomically —
/// `[| op1 op2 … |]`. Each inner operation carries its own verb
/// sigil; the batch succeeds only if every operation succeeds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "@NexusAtomicBatch")]
pub struct AtomicBatch<T>(pub T);
