defmodule CircularBufferRs.Native do
  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]

  use RustlerPrecompiled,
    otp_app: :circular_buffer_rs,
    crate: "circular_buffer_rs",
    base_url: "#{github_url}/releases/download/v#{version}",
    force_build: System.get_env("FORCE_CIRCULAR_BUFFER_RS_BUILD") in ["1", "true"],
    targets:
      Enum.uniq(["aarch64-unknown-linux-musl" | RustlerPrecompiled.Config.default_targets()]),
    # nif_versions: ["2.16"],
    version: version

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
