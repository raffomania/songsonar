# This file is responsible for configuring your application
# and its dependencies with the aid of the Mix.Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
import Config

config :exyarr,
  ecto_repos: [Exyarr.Repo],
  spotify_client_id: System.get_env("SPOTIFY_CLIENT_ID"),
  spotify_client_secret: System.get_env("SPOTIFY_CLIENT_SECRET")

# Configures the endpoint
config :exyarr, ExyarrWeb.Endpoint,
  url: [host: "localhost"],
  secret_key_base:
    "SKvMKjOp1o6siQ+JXHEZ6fWGXJKZIEq7MrkZGr5eo4orLBUktDdrlzUPRQpfWZzX",
  render_errors: [
    view: ExyarrWeb.ErrorView,
    accepts: ~w(html json),
    layout: false
  ],
  pubsub_server: Exyarr.PubSub,
  live_view: [signing_salt: "0kOBvuJr"]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{config_env()}.exs"
