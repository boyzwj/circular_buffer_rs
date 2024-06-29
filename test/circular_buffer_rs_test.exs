defmodule CircularBufferRsTest do
  use ExUnit.Case
  doctest CircularBufferRs

  test "new" do
    {:ok, ref} = CircularBufferRs.new(8)
    assert ref != nil
  end

  test "size" do
    {:ok, ref} = CircularBufferRs.new(8)
    assert CircularBufferRs.size(ref) == 8
  end

  test "push" do
    {:ok, ref} = CircularBufferRs.new(5)
    assert CircularBufferRs.push(ref, "hello") == {:ok, :ok}
  end

  test "last" do
    {:ok, ref} = CircularBufferRs.new(5)
    assert CircularBufferRs.push(ref, "1") == {:ok, :ok}
    assert CircularBufferRs.push(ref, "2") == {:ok, :ok}
    assert CircularBufferRs.push(ref, "3") == {:ok, :ok}
    assert CircularBufferRs.push(ref, "4") == {:ok, :ok}
    assert CircularBufferRs.push(ref, "5") == {:ok, :ok}
    assert CircularBufferRs.push(ref, "6") == {:ok, :ok}
    assert CircularBufferRs.last(ref, 2) == {:ok, [~c"5", ~c"6"]}
  end
end
