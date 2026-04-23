//! Tests for the three nexus-layer wrapper types: `Bind`, `Mutate<T>`,
//! `Negate<T>`. Also exercises that the nota-subset still roundtrips
//! unchanged through the nexus serializer.

use nexus_serde::{from_str, to_string, Bind, Mutate, Negate};
use serde::{Deserialize, Serialize};

#[test]
fn bind_roundtrip() {
    let b = Bind("h".into());
    let text = to_string(&b).unwrap();
    assert_eq!(text, "@h");
    let back: Bind = from_str(&text).unwrap();
    assert_eq!(back, b);
}

#[test]
fn bind_kebab_name() {
    let b = Bind("my-hole".into());
    assert_eq!(to_string(&b).unwrap(), "@my-hole");
    let back: Bind = from_str("@my-hole").unwrap();
    assert_eq!(back, b);
}

#[test]
fn bind_rejects_invalid_chars() {
    let b = Bind("has space".into());
    assert!(to_string(&b).is_err());
    let b = Bind("".into());
    assert!(to_string(&b).is_err());
}

#[test]
fn mutate_around_struct() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Point { horizontal: f64, vertical: f64 }
    let m = Mutate(Point { horizontal: 3.0, vertical: 4.0 });
    let text = to_string(&m).unwrap();
    assert_eq!(text, "~(Point 3.0 4.0)");
    let back: Mutate<Point> = from_str(&text).unwrap();
    assert_eq!(back, m);
}

#[test]
fn negate_around_unit_variant() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Status { Active, Archived }
    let n = Negate(Status::Active);
    let text = to_string(&n).unwrap();
    assert_eq!(text, "!Active");
    let back: Negate<Status> = from_str(&text).unwrap();
    assert_eq!(back, n);
}

#[test]
fn negate_around_primitive() {
    let n = Negate(42i32);
    assert_eq!(to_string(&n).unwrap(), "!42");
    let back: Negate<i32> = from_str("!42").unwrap();
    assert_eq!(back, n);
}

#[test]
fn mutate_of_negate_nests() {
    let v = Mutate(Negate(7i32));
    assert_eq!(to_string(&v).unwrap(), "~!7");
    let back: Mutate<Negate<i32>> = from_str("~!7").unwrap();
    assert_eq!(back, v);
}

#[test]
fn nota_subset_roundtrip() {
    // A representative nota document must parse identically via
    // nexus-serde since nexus is a strict superset of nota.
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Config { name: String, port: u16, flags: Vec<String> }

    let c = Config {
        name: "server".into(),
        port: 8080,
        flags: vec!["debug".into(), "verbose".into()],
    };
    let text = to_string(&c).unwrap();
    assert_eq!(text, "(Config [server] 8080 <[debug] [verbose]>)");
    let back: Config = from_str(&text).unwrap();
    assert_eq!(back, c);
}

#[test]
fn bind_inside_option() {
    // A bind hole nested inside an Option — serializes transparently
    // through the Option and emits @h.
    let v: Option<Bind> = Some(Bind("h".into()));
    assert_eq!(to_string(&v).unwrap(), "@h");
    let back: Option<Bind> = from_str("@h").unwrap();
    assert_eq!(back, v);
}

#[test]
fn new_delimiters_tokenize() {
    // The lexer must recognise (| |) { } {| |} even though we haven't
    // wired them to Rust types yet. We test this indirectly: they
    // appear as tokens the parser doesn't know how to consume, so
    // deserializing any non-trivial type should produce a parse error
    // rather than a lexer error.
    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Point { horizontal: f64 }

    // Valid nexus text containing a pattern — but the user asked for a
    // plain struct, so the deserializer should reject this at the
    // parser level (expected `(`, got `(|`), not crash in the lexer.
    let result: nexus_serde::Result<Point> = from_str("(| Point @horizontal |)");
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        !err.contains("unexpected character"),
        "should parse lexically; got {err:?}"
    );
}
