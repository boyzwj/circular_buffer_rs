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
  def push(cb, bin) when is_binary(bin) do
    Native.push(cb, bin)
  end

  def push(cb, io_list) when is_list(io_list) do
    Native.push(cb, IO.iodata_to_binary(io_list))
  end

  def push(_cb, _), do: {:error, :invalid_input}

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
