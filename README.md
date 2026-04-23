# nexus-serde

[serde](https://serde.rs) Serializer + Deserializer for the
[nexus](https://github.com/LiGoldragon/nexus) messaging protocol —
a strict superset of the [nota](https://github.com/LiGoldragon/nota)
data format.

nexus-serde handles:

- Everything [nota-serde](https://github.com/LiGoldragon/nota-serde)
  handles (records, strings, sequences, maps, enums, options,
  primitives).
- Three query-layer wrapper types: `Bind`, `Mutate<T>`, `Negate<T>`.
- Lexer tokens for the added delimiter pairs (`(| |)` patterns,
  `{| |}` constraints, `{ }` shapes) — mapping of these to Rust
  container types is deferred pending consumer-side design in
  [nexusd](https://github.com/LiGoldragon/nexusd) and
  [nexus-cli](https://github.com/LiGoldragon/nexus-cli).

## Usage

```rust
use nexus_serde::{Bind, Mutate, Negate};

#[derive(serde::Serialize, serde::Deserialize)]
struct Point { horizontal: f64, vertical: f64 }

let p = Point { horizontal: 3.0, vertical: 4.0 };

// Assert
let s = nexus_serde::to_string(&p)?;                    // (Point horizontal=3.0 vertical=4.0)

// Mutate
let s = nexus_serde::to_string(&Mutate(p))?;           // ~(Point horizontal=3.0 vertical=4.0)

// Bind hole
let b = Bind("h".into());
let s = nexus_serde::to_string(&b)?;                    // @h
```

For pure-data configs that don't need the query layer, use
[nota-serde](https://github.com/LiGoldragon/nota-serde) — same
grammar, smaller surface.

## License

[License of Non-Authority](LICENSE.md).
