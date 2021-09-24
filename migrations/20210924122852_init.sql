CREATE TABLE users (
       spotify_id text primary key not null,
       playlist_id text default null,
       access_token text not null,
       refresh_token text not null,
       weeks_in_playlist smallint not null
)

