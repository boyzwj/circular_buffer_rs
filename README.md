# CircularBufferRs

A **concurrent-safe** circular buffer backed by Rust NIF.

## Why this library?

Elixir's existing circular buffer implementations (e.g. `circular_buffer`) use immutable structs — fast in single-process scenarios, but each process must hold its own copy. Sharing state requires wrapping in an `Agent`, which serializes all access through a single process mailbox.

**CircularBufferRs** uses a `ResourceArc<RwLock<CircularBuffer>>` under the hood — multiple Elixir processes can hold a reference to the **same** buffer and operate concurrently:

- **Push** → acquires a write lock, only one writer at a time
- **Last / Size** → acquires a read lock, **multiple readers simultaneously**

No message passing. No single-process bottleneck.

## Usage

```elixir
iex> {:ok, buf} = CircularBufferRs.new(32)
iex> CircularBufferRs.push(buf, "hello")
:ok
iex> CircularBufferRs.push(buf, "world")
:ok
iex> CircularBufferRs.last(buf, 2)
{:ok, ["hello", "world"]}
iex> CircularBufferRs.size(buf)
32
```

## Installation

```elixir
def deps do
  [
    {:circular_buffer_rs, "~> 0.2.0"}
  ]
end
```

## Benchmark

### Single-process (struct copy vs NIF boundary)

| Operation | `circular_buffer` | `circular_buffer_rs` | Gap |
|-----------|-------------------|----------------------|------|
| Push | 18.30 M ips, 55 ns | 9.44 M ips, 106 ns | 1.9x slower |
| Last(5) | 4.22 M ips, 0.24 μs | 0.52 M ips, 1.93 μs | 8.1x slower |

In single-process context the pure-Elixir struct is faster — NIF boundary crossing + data serialization is fixed overhead.

### Multi-process (shared memory vs Agent)

**Concurrent Push:**

```
并发数 |      Rust NIF       |    Elixir Agent     | 加速比
--------|---------------------|---------------------|--------
     1 |      5.86M/s 0.17μs |      0.72M/s 1.38μs |  8.11x
     2 |      8.58M/s 0.12μs |       0.6M/s 1.67μs | 14.30x
     4 |       7.4M/s 0.14μs |      0.69M/s 1.45μs | 10.76x
     8 |      5.76M/s 0.17μs |      0.85M/s 1.18μs |  6.78x
    16 |      4.44M/s 0.23μs |      0.96M/s 1.05μs |  4.64x
```

Agent throughput is flat (~0.7M/s) regardless of concurrency — every operation pays message-passing cost.
Rust NIF acquires the write lock directly on the calling thread, **no message passing**, no mailbox bottleneck.

**Concurrent Mixed Push + Last:**

```
并发数 |      Rust NIF       |    Elixir Agent     | 加速比
--------|---------------------|---------------------|--------
     1 |     0.02M/s 60.51μs |     0.0M/s 241.86μs |  4.00x
     4 |     0.04M/s 27.16μs |    0.01M/s 192.83μs |  7.10x
    16 |      0.05M/s 19.7μs |    0.01M/s 187.86μs |  9.54x
```

With mixed read/write workloads the Agent pays double message-passing cost (update + get), while Rust's `RwLock` allows reads to proceed in parallel with writes.

**Concurrent Read (Last):**

```
并发数 |      Rust NIF       |    Elixir Agent     | 加速比
--------|---------------------|---------------------|--------
     1 |     0.02M/s 64.54μs |     0.02M/s 49.27μs |  0.76x
     8 |     0.05M/s 18.69μs |     0.03M/s 28.59μs |  1.53x
    16 |     0.06M/s 17.94μs |     0.03M/s 29.29μs |  1.63x
```

Rust's RwLock read lock scales with concurrency (64μs → 18μs). Agent latency stays flat — the mailbox is the bottleneck.

**System:** 12th Gen i7-12700K (20 cores), Elixir 1.19.5, Erlang 28.5, JIT enabled.

## When to use which

| Scenario | Reach for `circular_buffer` | Reach for `circular_buffer_rs` |
|----------|---------------------------|-------------------------------|
| Single-process buffer | ✅ Fastest option | ❌ |
| Shared state across processes | ❌ Needs Agent wrapper | ✅ True concurrent access |
| High-read-throughput | ❌ Agent serializes reads | ✅ RwLock scales readers |
| Mixed read/write across processes | ❌ | ✅ |
| Zero memory allocation | ❌ Struct copies on every write | ✅ In-place mutation |

## Supported term types

Integers, atoms, strings/binaries, tuples, lists.

Not supported: References, functions, ports, PIDs.
