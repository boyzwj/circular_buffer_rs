defmodule CircularBufferRs.Native do
  use Rustler,
    otp_app: :circular_buffer_rs,
    crate: "circular_buffer_rs"

  def new(_size) do
    :erlang.nif_error(:nif_not_loaded)
  end

  def push(_cb, _bytes) do
    :erlang.nif_error(:nif_not_loaded)
  end

  def last(_cb, _count) do
    :erlang.nif_error(:nif_not_loaded)
  end

  def size(_cb) do
    :erlang.nif_error(:nif_not_loaded)
  end
end
