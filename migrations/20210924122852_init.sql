CREATE TABLE IF NOT EXISTS users (
       spotify_id text primary key not null,
       playlist_id text default null,
       access_token text not null,
       refresh_token text not null,
       can_read_private_playlists boolean,
       weeks_in_playlist smallint default 1
)

