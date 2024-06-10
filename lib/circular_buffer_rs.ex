defmodule CircularBufferRs do
  alias CircularBufferRs.Native

  @doc """
  Creates a new buffer with the given UID and size.
  """
  def new(uid, size \\ 32) do
    Native.new(uid, size)
  end

  @doc """
  Pushes the given bytes to the buffer with the given UID.
  """
  def push(uid, bin) when is_binary(bin) do
    Native.push(uid, bin)
  end

  def push(uid, io_list) when is_list(io_list) do
    Native.push(uid, IO.iodata_to_binary(io_list))
  end

  def push(_uid, _), do: {:error, :invalid_input}

  @doc """
  Returns the last `length` elements from the buffer with the given UID.
  """
  def last(uid, length) do
    Native.last(uid, length)
  end

  @doc """
  Removes the buffer with the given UID.
  """
  def remove(uid) do
    Native.remove(uid)
  end
end
