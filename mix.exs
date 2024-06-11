defmodule CircularBufferRs.MixProject do
  use Mix.Project
  @version "0.1.0"
  @source_url "https://github.com/boyzwj/circular_buffer_rs"
  def project do
    [
      app: :circular_buffer_rs,
      version: @version,
      elixir: "~> 1.16",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      elixirc_paths: elixirc_paths(Mix.env()),
      package: package()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp elixirc_paths(:test) do
    elixirc_paths(:default) ++ ["test/support"]
  end

  defp elixirc_paths(_) do
    ["lib"]
  end

  defp package do
    [
      maintainers: ["Discord Core Infrastructure"],
      licenses: ["MIT"],
      name: :circular_buffer_rs,
      description: "CircularBufferRs is a fast and efficient Rust backed circular buffer.",
      files: [
        "lib",
        "native",
        "checksum-*.exs",
        "priv/.gitkeep",
        "mix.exs",
        ".formatter.exs",
        "README*",
        "LICENSE*"
      ],
      links: %{"GitHub" => @source_url}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
      {:rustler_precompiled, "~> 0.7.0"},
      {:rustler, "~> 0.23", optional: true},
      {:benchee, "~> 1.3", only: [:test]},
      {:circular_buffer, "~> 0.4.1", only: [:test]}
    ]
  end
end
