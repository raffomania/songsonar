defmodule Exyarr.Repo.Migrations.CreateUsers do
  use Ecto.Migration

  def change do
    create table(:users) do
      add :spotify_id, :string
      add :playlist_id, :string

      timestamps()
    end

  end
end
