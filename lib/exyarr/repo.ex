defmodule Exyarr.Repo do
  use Ecto.Repo,
    otp_app: :exyarr,
    adapter: Ecto.Adapters.Postgres
end
