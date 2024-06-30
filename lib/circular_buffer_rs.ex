defmodule CircularBufferRs do
  alias CircularBufferRs.Native

  @default_capacity 32

  @doc """
  Creates a new buffer with the given size.
  """
  def new(size \\ @default_capacity) do
    Native.new(size)
  end

  @doc """
  Pushes the given bytes to the buffer.
  """
  def push(cb, bin) do
    Native.push(cb, bin)
  end

  @doc """
  Returns the last `length` elements from the buffer .
  """
  def last(cb, length) do
    Native.last(cb, length)
  end

  @doc """
  Returns the capacity of the buffer.
  """
  def size(cb) do
    {:ok, size} = Native.size(cb)
    size
  end
end
