# CircularBufferRs

**TODO: Add description**

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `circular_buffer_rs` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:circular_buffer_rs, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/circular_buffer_rs>.


## Benchmark

```shell

Operating System: Linux
CPU Information: AMD Ryzen 7 5800H with Radeon Graphics
Number of Available Cores: 16
Available memory: 31.20 GB
Elixir 1.16.3
Erlang 25.3.2.12
JIT enabled: true

Benchmark suite executing with the following configuration:
warmup: 1 s
time: 5 s
memory time: 2 s
reduction time: 2 s
parallel: 1
inputs: none specified
Estimated total run time: 20 s

Benchmarking circular_buffer_add ...
Benchmarking circular_buffer_rs_add ...
Calculating statistics...
Formatting results...

Name                             ips        average  deviation         median         99th %
circular_buffer_rs_add        6.16 M      162.32 ns   ±115.76%         140 ns         430 ns
circular_buffer_add           4.59 M      217.87 ns ±10213.66%         180 ns         641 ns

Comparison: 
circular_buffer_rs_add        6.16 M
circular_buffer_add           4.59 M - 1.34x slower +55.55 ns

Memory usage statistics:

Name                      Memory usage
circular_buffer_rs_add            24 B
circular_buffer_add               80 B - 3.33x memory usage +56 B

**All measurements for memory usage were the same**

Reduction count statistics:

Name                   Reduction count
circular_buffer_rs_add               1
circular_buffer_add                  1 - 1.00x reduction count +0

**All measurements for reduction count were the same**

```