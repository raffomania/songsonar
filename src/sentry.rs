use log::Log;
use pretty_env_logger::env_logger::Logger;
use sentry::ClientInitGuard;

/// Wrap the given logger with a sentry listener that forwards log entries to
/// sentry.
pub fn get_logger(logger: Logger) -> impl Log {
    let sentry_logger =
        sentry::integrations::log::SentryLogger::with_dest(logger);

    sentry_logger
}

/// Initialize sentry.
/// Hold on to the returned guard of this function as long as
/// the app is running, because sentry only captures events until the guard
/// is dropped.
pub fn init() -> ClientInitGuard {
    sentry::init((
        std::env::var("SENTRY_DSN")
            .expect("Please set the SENTRY_DSN environment variable"),
        sentry::ClientOptions {
            release: Some(git_version::git_version!().into()),
            ..Default::default()
        },
    ))
}
