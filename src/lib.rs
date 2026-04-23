//! nexus-serde — serde Serializer + Deserializer for the nexus data format.
//!
//! Implements [`serde::Serializer`] and [`serde::Deserializer`] over the
//! nexus syntax: 6 delimiter pairs, 4 sigils, Pascal/camel/kebab
//! identifiers, literal forms. Any type implementing `Serialize` +
//! `Deserialize` can round-trip through nexus text.
