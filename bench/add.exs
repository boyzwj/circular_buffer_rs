
a = CircularBuffer.new(32)
CircularBufferRs.new(1, 32)

Benchee.run(%{
  "circular_buffer_add" => fn ->CircularBuffer.insert(a,"gdfgdgdfgdfgdfgdfgdgdgd") end,
  "circular_buffer_rs_add" => fn -> CircularBufferRs.push(1, "gdfgdgdfgdfgdfgdfgdgdgd") end
},
  warmup: 1,
  time: 5,
  memory_time: 2,
  reduction_time: 2)
