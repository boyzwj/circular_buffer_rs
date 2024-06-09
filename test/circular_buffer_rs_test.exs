defmodule CircularBufferRsTest do
  use ExUnit.Case
  doctest CircularBufferRs

  test "new" do
    assert CircularBufferRs.new(1, 5) == {:ok, :ok}
  end

  test "push" do
    {:ok, :ok} = CircularBufferRs.new(1, 5)
    assert CircularBufferRs.push(1, "hello") == {:ok, :ok}
  end

  test "remove" do
    {:ok, :ok} = CircularBufferRs.new(1, 5)
    assert CircularBufferRs.remove(1) == {:ok, :ok}
  end

  test "last" do
    {:ok, :ok} = CircularBufferRs.new(1, 5)
    assert CircularBufferRs.push(1, "1") == {:ok, :ok}
    assert CircularBufferRs.push(1, "2") == {:ok, :ok}
    assert CircularBufferRs.push(1, "3") == {:ok, :ok}
    assert CircularBufferRs.push(1, "4") == {:ok, :ok}
    assert CircularBufferRs.push(1, "5") == {:ok, :ok}
    assert CircularBufferRs.push(1, "6") == {:ok, :ok}
    assert CircularBufferRs.last(1, 2) == {:ok, [~c"5", ~c"6"]}
  end
end
