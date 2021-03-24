defmodule Exyarr.Repo.Migrations.CreateUsers do
  use Ecto.Migration

  def change do
    create table(:users, primary_key: false) do
      add :spotify_id, :text, null: false, primary_key: true
      add :playlist_id, :text
      add :access_token, :text, null: false
      add :refresh_token, :text, null: false
      add :weeks_in_playlist, :smallint, null: false, default: 1
    end
  end
end
