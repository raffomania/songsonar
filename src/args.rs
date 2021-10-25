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
}

#[derive(FromArgs)]
#[argh(subcommand, name = "migrate")]
/// Migrate the database
pub struct MigrateOptions {}

#[derive(FromArgs)]
#[argh(subcommand, name = "start")]
/// Start the server
pub struct StartOptions {}
