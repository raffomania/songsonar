defmodule Exyarr.User do
  use Ecto.Schema
  import Ecto.Changeset

  schema "users" do
    field :playlist_id, :string
    field :spotify_id, :string

    timestamps()
  end

  @doc false
  def changeset(user, attrs) do
    user
    |> cast(attrs, [:spotify_id, :playlist_id])
    |> validate_required([:spotify_id, :playlist_id])
  end
end
