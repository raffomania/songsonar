defmodule Exyarr.User do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:spotify_id, :string, []}
  schema "users" do
    field :playlist_id, :string
    field :access_token, :string
    field :refresh_token, :string
    field :weeks_in_playlist, :integer, default: 1
  end

  @doc false
  def changeset(user, attrs) do
    user
    |> cast(attrs, [
      :spotify_id,
      :playlist_id,
      :access_token,
      :refresh_token,
      :weeks_in_playlist
    ])
    |> validate_required([:spotify_id, :access_token, :refresh_token])
  end
end
