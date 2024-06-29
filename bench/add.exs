
a = CircularBuffer.new(64)
{:ok,b} = CircularBufferRs.new(64)
buffer = "gdfgfdgdfgfdgdfgfdgdfgdfgfdgfdgdfgdfgdfgdfgfdgdfgfdgfdgdfgdfgdfgdfgdfgdfgfdgdfgdfgdfgdfgdgdfghgfhghfghfghgfgffghgfhgfgfdgdfgdfgfdgdfdfgdfgdfgdfgdfgdfgfdgdfgfdgdf"

Benchee.run(%{
  "circular_buffer_add" => fn ->CircularBuffer.insert(a, buffer) end,
  "circular_buffer_rs_add" => fn -> CircularBufferRs.push(b, buffer) end
},
  warmup: 1,
  time: 5,
  memory_time: 2,
  reduction_time: 2)
