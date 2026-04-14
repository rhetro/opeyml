# Opeyml

**A declarative, zero-overhead YAML surgery DSL for Rust.**

`opeyml` provides precise, Perl-like auto-vivification (automatically creating intermediate mappings and sequences) for `serde_yaml::Value` while bypassing runtime string parsing entirely. Every `.foo.bar[3].baz` traversal is resolved at compile time into static pointer chains, reducing complex topological mutations down to pure memory operations.

> **Note:** `opeyml` is the YAML counterpart to [`opejson`](https://github.com/rhetro/opejson). Both share the exact same zero-overhead architectural principles and surgical DSL, differentiated only by their target underlying AST.

## The Traversal Bottleneck

Manually mutating nested `serde_yaml::Value` trees in Rust requires deep `Option` matching, verbose error handling, and runtime path parsing.

`opeyml` eliminates this abstraction cost. By utilizing declarative macros, it compiles traversal paths into direct `get_mut()` sequences. The execution time is strictly bounded by the physical memory allocation of the underlying `IndexMap`, as the path routing overhead is mathematically reduced to zero.

## Performance Proof: Zero-Overhead Routing

Because `opeyml` does not parse string paths at runtime, it completely bypasses the traditional latency of dynamic data traversal. All traversal cost is resolved at compile time; runtime execution reduces to direct pointer chasing inside `IndexMap`.

**Raw Throughput Benchmark (Release Mode):**
```text
=======================================================================
[ OPEYML : ABSOLUTE THROUGHPUT VISUALIZED ]
=======================================================================
📍 256-Level Penetration: 424 µs
🧵 100,000 Massive Sutures: 48.12 ms
-----------------------------------------------------------------------
🏆 Raw Throughput: 2,064,992 operations/sec
=======================================================================
```
*Note: Reaching a depth of 256 levels completes in ~424µs. The 48ms spent on 100,000 sutures is almost entirely consumed by the inevitable heap allocation of `IndexMap::new()`. The calculation cost of path traversal is effectively zero.*

## Architectural Boundaries

To prevent accidental data destruction, `opeyml` categorizes operations into two strict modules.

### 1. Genesis Mode (`opeyml::genesis::*`)
Designed for structural creation, expansion, and merging.

* **`suture!`**: Safe auto-vivification. Generates missing paths but will **not** destroy existing scalars to build them.
* **`force_suture!`**: Destructive auto-vivification. **Will** overwrite existing scalars to force path creation.
* **`implant!`**: Non-destructive void filler. Only writes to `Null` spaces.
* **`graft!`**: Structural merge/append. Pushes to arrays or extends mappings.
* **`mesh!`**: O(1) multi-dimensional matrix allocation.

### 2. Strict Mode (`opeyml::strict::*`)
Designed for precision surgery on existing topologies. Strict mode never creates structure; all operations assume the full path already exists.

* **`biopsy!`**: Zero-allocation read. Returns `Option<&Value>`.
* **`acquire!`**: Strict read. Returns `Result<&Value, Error>`, pinpointing the exact severed node on failure.
* **`incise!`**: Strict write. Succeeds **only** if the path already exists.
* **`excise!`**: Removes and returns a specific node from the tree.

## Quick Start

Add `opeyml` to your `Cargo.toml`:

```toml
[dependencies]
opeyml = "0.1.0"
serde_yaml = "0.9"
```

### Deep Mutation (Genesis)
```rust
use opeyml::genesis::*;
use serde_yaml::Value;

fn main() {
    let mut doc = Value::Null;
    // Automatically builds the mapping structure
    suture!(doc, .metadata.name = "nginx");
    suture!(doc, .spec.containers[0].image = "nginx:latest");

    // Appends to the sequence
    graft!(doc, .spec.containers[0].env = [ { "RUST_LOG": "debug" } ]);
}
```

### Precision Read & Write (Strict)
```rust
use opeyml::strict::*;
use serde_yaml::Value;
fn main() {
    let mut doc = Value::Null;
    // Zero-allocation read
    if let Some(version) = biopsy!(&doc, .system.kernel.version) {
        println!("Kernel: {}", version.as_str().unwrap());
}

    // Strict update (fails silently if the path does not exist)
    let _ = incise!(doc, .system.kernel.version = "6.2.0");
}
```

## Heterogeneous Key Routing

`opeyml` supports mixing identifiers, literals, array indices, and dynamic expressions in a single static chain:

* `.ident` — Standard identifier (`.metadata`)
* `."literal"` — String literal (`."http-status"`)
* `.404` — Numeric map key (`.404`)
* `.(expr)` — Dynamic runtime expression (`.(var_name)`)
* `[index]` — Sequence index (`[0]`)

```rust
use opeyml::genesis::*;
use serde_yaml::Value;
fn main() {
    let mut doc = Value::Null;
    let trace_id = "x-request-id";

// Mixes identifiers, dynamic variables, and numeric keys perfectly
suture!(doc, .metadata.annotations.(trace_id) = "0xdeadbeef");
suture!(doc, .openapi.responses.404.description = "Not Found");
# }
```

## Comparison with Standard `serde_yaml`

| Operation | Standard `serde_yaml::Value` | `opeyml` |
| :--- | :--- | :--- |
| **Deep Path Write** | 10+ lines of `match` / `as_mapping_mut` | `suture!(doc, .a.b.c = 1);` |
| **Safe Read** | Nested `if let` chains | `biopsy!(&doc, .a.b.c)` |
| **Strict Update** | Manual `Option` checks | `incise!(doc, .a.b.c = 1);` |
| **Array Append** | `.as_sequence_mut().unwrap().push(...)` | `graft!(doc, .arr = [1]);` |
| **Multi-dim Array** | Hand-written loops / nested `vec!` | `mesh!(doc, [3][3]);` |

## License

This project is licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.
