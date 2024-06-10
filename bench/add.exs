
a = CircularBuffer.new(320)
CircularBufferRs.new(1, 320)

Benchee.run(%{
  "circular_buffer_add" => fn ->CircularBuffer.insert(a,"gdfgfdgdfgfdgdfgfdgdfgdfgfdgfdgdfgdfgdfgdfgfdgdfgfdgfdgdfgdfgdfgdfgdfgdfgfdgdfgdfgdfgdfgdgdfg") end,
  "circular_buffer_rs_add" => fn -> CircularBufferRs.push(1, "gdfgfdgdfgfdgdfgfdgdfgdfgfdgfdgdfgdfgdfgdfgfdgdfgfdgfdgdfgdfgdfgdfgdfgdfgfdgdfgdfgdfgdfgdgdfg") end
},
  warmup: 1,
  time: 5,
  memory_time: 2,
  reduction_time: 2)
