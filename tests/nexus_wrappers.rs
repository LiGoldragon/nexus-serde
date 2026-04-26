//! Tests for the six nexus-layer wrapper types: `Bind`, `Mutate<T>`,
//! `Negate<T>`, `Validate<T>`, `Subscribe<T>`, `AtomicBatch<T>`. Also
//! exercises that the nota-subset still roundtrips unchanged through
//! the nexus serializer.

use nexus_serde::{
    from_str, to_string, AtomicBatch, Bind, Mutate, Negate, Subscribe, Validate,
};
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
fn bind_rejects_leading_digit() {
    // Digits can appear in the body but not at the start — reserves
    // numeric-first for integer literals.
    let b = Bind("123foo".into());
    assert!(to_string(&b).is_err());
}

#[test]
fn bind_rejects_leading_hyphen() {
    // `-` is valid inside kebab-case but not as the first char.
    let b = Bind("-foo".into());
    assert!(to_string(&b).is_err());
}

#[test]
fn bind_rejects_uppercase() {
    // PascalCase-shaped names are reserved for types and variants;
    // a bind hole can't take that form.
    let b = Bind("Foo".into());
    assert!(to_string(&b).is_err());
    let b = Bind("fooBar".into());
    assert!(to_string(&b).is_err());
}

#[test]
fn bind_accepts_leading_underscore() {
    // Per the nota spec (identifier classes), a leading `_` counts
    // as camelCase-kindred — matches Rust's convention for
    // "nominally-private" field names.
    let b = Bind("_private".into());
    assert_eq!(to_string(&b).unwrap(), "@_private");
    let back: Bind = from_str("@_private").unwrap();
    assert_eq!(back, b);
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
    // Ident-shaped strings emit bare (inherited from nota-serde rules).
    assert_eq!(text, "(Config server 8080 [debug verbose])");
    let back: Config = from_str(&text).unwrap();
    assert_eq!(back, c);
}

#[test]
fn validate_around_struct() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Point { horizontal: f64, vertical: f64 }
    let v = Validate(Point { horizontal: 3.0, vertical: 4.0 });
    let text = to_string(&v).unwrap();
    assert_eq!(text, "?(Point 3.0 4.0)");
    let back: Validate<Point> = from_str(&text).unwrap();
    assert_eq!(back, v);
}

#[test]
fn subscribe_around_record() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Node { id: String, label: String }
    let s = Subscribe(Node { id: "u".into(), label: "User".into() });
    let text = to_string(&s).unwrap();
    assert_eq!(text, "*(Node u User)");
    let back: Subscribe<Node> = from_str(&text).unwrap();
    assert_eq!(back, s);
}

#[test]
fn atomic_batch_of_mutates() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Node { id: String, label: String }
    let batch = AtomicBatch(vec![
        Mutate(Node { id: "a".into(), label: "Apple".into() }),
        Mutate(Node { id: "b".into(), label: "Banana".into() }),
    ]);
    let text = to_string(&batch).unwrap();
    assert_eq!(text, "[| ~(Node a Apple) ~(Node b Banana) |]");
    let back: AtomicBatch<Vec<Mutate<Node>>> = from_str(&text).unwrap();
    assert_eq!(back, batch);
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

mod char_tests {
    use nexus_serde::{from_str, to_string};

    #[test]
    fn char_roundtrip_nexus() {
        let original = 'a';
        let text = to_string(&original).unwrap();
        let back: char = from_str(&text).unwrap();
        assert_eq!(back, original);
    }
}

// ---------------------------------------------------------------------------
// Additional coverage for the new wrappers.

mod validate {
    use super::*;

    #[test]
    fn validate_around_int() {
        let v = Validate(5i32);
        let text = to_string(&v).unwrap();
        assert_eq!(text, "?5");
        let back: Validate<i32> = from_str(&text).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn validate_around_unit_variant() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum Status { Pending }
        let v = Validate(Status::Pending);
        assert_eq!(to_string(&v).unwrap(), "?Pending");
        let back: Validate<Status> = from_str("?Pending").unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn validate_around_string() {
        let v = Validate("hello".to_string());
        assert_eq!(to_string(&v).unwrap(), "?hello");
        let back: Validate<String> = from_str("?hello").unwrap();
        assert_eq!(back, v);
    }
}

mod subscribe {
    use super::*;

    #[test]
    fn subscribe_around_int() {
        let v = Subscribe(7i32);
        let text = to_string(&v).unwrap();
        assert_eq!(text, "*7");
        let back: Subscribe<i32> = from_str(&text).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn subscribe_around_unit_variant() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum Status { Live }
        let v = Subscribe(Status::Live);
        assert_eq!(to_string(&v).unwrap(), "*Live");
        let back: Subscribe<Status> = from_str("*Live").unwrap();
        assert_eq!(back, v);
    }
}

mod atomic_batch {
    use super::*;

    #[test]
    fn empty_atomic_batch() {
        let b: AtomicBatch<Vec<i32>> = AtomicBatch(vec![]);
        let text = to_string(&b).unwrap();
        assert_eq!(text, "[||]");
        let back: AtomicBatch<Vec<i32>> = from_str(&text).unwrap();
        assert_eq!(back, b);
    }

    #[test]
    fn atomic_batch_of_ints() {
        let b = AtomicBatch(vec![1, 2, 3]);
        let text = to_string(&b).unwrap();
        assert_eq!(text, "[| 1 2 3 |]");
        let back: AtomicBatch<Vec<i32>> = from_str(&text).unwrap();
        assert_eq!(back, b);
    }

    #[test]
    fn atomic_batch_with_negate_and_mutate() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Node { id: String, label: String }
        // Three different verbs in one batch — each item carries its
        // own sigil; the batch is heterogeneous in sigil but
        // homogeneous in T at the type level. To mix verbs in serde,
        // we wrap each item the same way — here all are Mutate.
        let b = AtomicBatch(vec![
            Mutate(Node { id: "a".into(), label: "Apple".into() }),
            Mutate(Node { id: "b".into(), label: "Banana".into() }),
            Mutate(Node { id: "c".into(), label: "Cherry".into() }),
        ]);
        let text = to_string(&b).unwrap();
        assert_eq!(
            text,
            "[| ~(Node a Apple) ~(Node b Banana) ~(Node c Cherry) |]"
        );
        let back: AtomicBatch<Vec<Mutate<Node>>> = from_str(&text).unwrap();
        assert_eq!(back, b);
    }
}

mod nested_wrappers {
    use super::*;

    #[test]
    fn validate_of_mutate() {
        let v = Validate(Mutate(7i32));
        let text = to_string(&v).unwrap();
        assert_eq!(text, "?~7");
        let back: Validate<Mutate<i32>> = from_str(&text).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn subscribe_of_negate() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum Status { Active }
        let v = Subscribe(Negate(Status::Active));
        let text = to_string(&v).unwrap();
        assert_eq!(text, "*!Active");
        let back: Subscribe<Negate<Status>> = from_str(&text).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn validate_of_atomic_batch() {
        let v = Validate(AtomicBatch(vec![1, 2, 3]));
        let text = to_string(&v).unwrap();
        assert_eq!(text, "?[| 1 2 3 |]");
        let back: Validate<AtomicBatch<Vec<i32>>> = from_str(&text).unwrap();
        assert_eq!(back, v);
    }
}
