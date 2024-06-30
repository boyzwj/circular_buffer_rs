
a = CircularBuffer.new(32)
{:ok, b} = CircularBufferRs.new(32)
buffer = "gdfgfdgdfgfdgdfgfdgdfgdfgfdgfdgdfgdfgdfgdfgfdgdfgfdgfdgdfgdfgdfgdfgdfgdfgfdgdfgdfgdfgdfgdgdfghgfhghfghfghgfgffghgfhgfgfdgdfgdfgfdgdfdfgdfgdfgdfgdfgdfgfdgdfgfdgdf"

a = Enum.reduce(1..100, a, fn _, acc -> CircularBuffer.insert(acc, buffer) end)
Enum.each(1..100, fn _ -> CircularBufferRs.push(b, buffer) end)

Benchee.run(%{
  "circular_buffer_last" => fn -> CircularBuffer.to_list(a)|> Enum.take(-5) end,
  "circular_buffer_rs_last" => fn -> CircularBufferRs.last(b, 5) end },
  warmup: 1,
  time: 5,
  memory_time: 2,
  reduction_time: 2)
