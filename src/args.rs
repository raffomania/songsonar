use argh::FromArgs;

#[derive(FromArgs)]
/// Song Sonar server management CLI
pub struct Args {
    #[argh(subcommand)]
    pub command: Option<Command>,
}

#[derive(FromArgs)]
#[argh(subcommand)]
/// The command to execute
pub enum Command {
    Migrate(MigrateOptions),
    Start(StartOptions),
    UpdatePlaylist(UpdatePlaylistOptions),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "migrate")]
/// Migrate the database
pub struct MigrateOptions {}

#[derive(FromArgs)]
#[argh(subcommand, name = "start")]
/// Start the server
pub struct StartOptions {}

#[derive(FromArgs)]
#[argh(subcommand, name = "update_playlist")]
/// Update a specific playlist
pub struct UpdatePlaylistOptions {
    #[argh(positional)]
    pub user_id: String,
}
