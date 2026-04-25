# ARCHITECTURE — nexus-serde

The public façade for nexus-text serde. Consumers — anything
that wants to round-trip Rust types through nexus text — depend
on this crate, not on the kernel directly.

## Role

Thin wrapper over
[nota-serde-core](https://github.com/LiGoldragon/nota-serde-core)
configured at `Dialect::Nexus`. Re-exports the serde entry points
and provides nexus-specific knobs.

## Boundaries

Owns:

- `from_str` / `to_string` entry points pinned to nexus
  dialect.
- Display / debug formatters specific to nexus text style.

Does not own:

- Tokenisation or parsing — that's nota-serde-core.
- The grammar itself — that's
  [nexus](https://github.com/LiGoldragon/nexus).
- The signal envelope — that's
  [signal](https://github.com/LiGoldragon/signal). nexus-serde
  produces parsed AST; the daemon turns it into signal frames.

## Status

CANON. Stable façade.

## Cross-cutting context

- Layer 0 of the project:
  [criome/ARCHITECTURE.md §8](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)
