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
    assert CircularBufferRs.push(ref, "hello") == :ok
  end

  test "last" do
    {:ok, ref} = CircularBufferRs.new(5)
    assert CircularBufferRs.push(ref, "1") == :ok
    assert CircularBufferRs.push(ref, "2") == :ok
    assert CircularBufferRs.push(ref, "3") == :ok
    assert CircularBufferRs.push(ref, "4") == :ok
    assert CircularBufferRs.push(ref, 5) == :ok
    assert CircularBufferRs.push(ref, :a) == :ok
    assert CircularBufferRs.last(ref, 2) == {:ok, [5, :a]}
  end
end
