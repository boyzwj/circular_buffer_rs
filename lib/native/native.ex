defmodule CircularBufferRs.Native do
  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]

  use RustlerPrecompiled,
    otp_app: :circular_buffer_rs,
    crate: "circular_buffer_rs",
    base_url: "#{github_url}/releases/download/v#{version}",
    force_build: System.get_env("FORCE_CIRCULAR_BUFFER_RS_BUILD") in ["1", "true"],
    targets: [
      "aarch64-unknown-linux-gnu",
      "aarch64-unknown-linux-musl",
      "arm-unknown-linux-gnueabihf",
      "riscv64gc-unknown-linux-gnu",
      "x86_64-unknown-linux-gnu",
      "x86_64-unknown-linux-musl"
    ],
    # nif_versions: ["2.16"],
    version: version

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
