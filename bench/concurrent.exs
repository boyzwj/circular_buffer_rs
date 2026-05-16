data = String.duplicate("x", 128)
ops_per_task = 10_000

measure = fn concurrency, fun ->
  tasks = for _ <- 1..concurrency do
    Task.async(fn ->
      for _ <- 1..ops_per_task, do: fun.()
    end)
  end
  {time_us, _} = :timer.tc(fn -> Task.await_many(tasks, :infinity) end)
  total_ops = concurrency * ops_per_task
  ops_per_sec = total_ops / (time_us / 1_000_000)
  us_per_op = time_us / total_ops
  {ops_per_sec, us_per_op}
end

print_header = fn ->
  IO.puts(" 并发数 |      Rust NIF       |    Elixir Agent     | 加速比")
  IO.puts("--------|---------------------|---------------------|--------")
end

print_row = fn c, rust_ops, rust_us, agent_ops, agent_us ->
  ratio = Float.round(rust_ops / agent_ops, 2)
  IO.puts(" #{String.pad_leading("#{c}", 6)} | #{String.pad_leading("#{Float.round(rust_ops / 1_000_000, 2)}M/s #{Float.round(rust_us, 2)}μs", 19)} | #{String.pad_leading("#{Float.round(agent_ops / 1_000_000, 2)}M/s #{Float.round(agent_us, 2)}μs", 19)} | #{ratio}x")
end

IO.puts("====================")
IO.puts(" 并发 Push 基准测试")
IO.puts("====================")
IO.puts("")

print_header.()

for c <- [1, 2, 4, 8, 16] do
  {:ok, agent} = Agent.start_link(fn -> CircularBuffer.new(4096) end)
  {:ok, rust} = CircularBufferRs.new(4096)

  {rust_ops, rust_us} = measure.(c, fn -> CircularBufferRs.push(rust, data) end)
  {agent_ops, agent_us} = measure.(c, fn -> Agent.update(agent, &CircularBuffer.insert(&1, data)) end)
  print_row.(c, rust_ops, rust_us, agent_ops, agent_us)
end

IO.puts("")
IO.puts("====================")
IO.puts(" 并发 Last 基准测试")
IO.puts("（last(5)读取最近5项）")
IO.puts("====================")
IO.puts("")

print_header.()

for c <- [1, 2, 4, 8, 16] do
  {:ok, agent} = Agent.start_link(fn -> CircularBuffer.new(4096) end)
  {:ok, rust} = CircularBufferRs.new(4096)

  Enum.each(1..512, fn _ ->
    CircularBufferRs.push(rust, data)
    Agent.update(agent, &CircularBuffer.insert(&1, data))
  end)

  {rust_ops, rust_us} = measure.(c, fn -> CircularBufferRs.last(rust, 5) end)
  {agent_ops, agent_us} = measure.(c, fn -> CircularBuffer.to_list(Agent.get(agent, & &1)) |> Enum.take(-5) end)
  print_row.(c, rust_ops, rust_us, agent_ops, agent_us)
end

IO.puts("")
IO.puts("====================")
IO.puts(" 混合 Push + Last")
IO.puts("（每个任务交替 push 和 last）")
IO.puts("====================")
IO.puts("")

print_header.()

for c <- [1, 2, 4, 8, 16] do
  {:ok, agent} = Agent.start_link(fn -> CircularBuffer.new(4096) end)
  {:ok, rust} = CircularBufferRs.new(4096)

  Enum.each(1..512, fn _ ->
    CircularBufferRs.push(rust, data)
    Agent.update(agent, &CircularBuffer.insert(&1, data))
  end)

  {rust_ops, rust_us} = measure.(c, fn ->
    CircularBufferRs.push(rust, data)
    CircularBufferRs.last(rust, 5)
  end)
  {agent_ops, agent_us} = measure.(c, fn ->
    Agent.update(agent, &CircularBuffer.insert(&1, data))
    CircularBuffer.to_list(Agent.get(agent, & &1)) |> Enum.take(-5)
  end)
  print_row.(c, rust_ops, rust_us, agent_ops, agent_us)
end
