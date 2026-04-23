# nexus-serde

[serde](https://serde.rs) Serializer + Deserializer for the nexus
data format syntax. Analogous to `serde_json` but for nexus.

## Usage

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct Point { horizontal: f64, vertical: f64 }

let p = Point { horizontal: 3.0, vertical: 4.0 };
let text = nexus_serde::to_string(&p)?;
let back: Point = nexus_serde::from_str(&text)?;
```

## License

[License of Non-Authority](LICENSE.md).
