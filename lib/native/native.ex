defmodule CircularBufferRs.Native do
  use Rustler, otp_app: :circular_buffer_rs, crate: :circular_buffer_rs

  def new(_uid, _size) do
    :erlang.nif_error(:nif_not_loaded)
  end

  def push(_uid, _bytes) do
    :erlang.nif_error(:nif_not_loaded)
  end

  def last(_uid, _count) do
    :erlang.nif_error(:nif_not_loaded)
  end

  def remove(_uid) do
    :erlang.nif_error(:nif_not_loaded)
  end
end
