use std::sync::Arc;

use log::Log;
use pretty_env_logger::env_logger::Logger;
use sentry::ClientInitGuard;

/// Wrap the given logger with a sentry listener that forwards log entries to
/// sentry.
pub fn get_logger(logger: Logger) -> impl Log {
    sentry::integrations::log::SentryLogger::with_dest(logger)
}

/// Initialize sentry.
/// Hold on to the returned guard of this function as long as
/// the app is running, because sentry only captures events until the guard
/// is dropped.
pub fn init() -> ClientInitGuard {
    let version = git_version::git_version!(args = ["--tags", "--always"]);
    sentry::init(sentry::ClientOptions {
        release: Some(version.into()),
        send_default_pii: false,
        attach_stacktrace: true,
        before_send: Some(Arc::new(|event| {
            let is_404 = event
                .message
                .as_ref()
                .map(|msg| msg.starts_with("No matching routes for"))
                .unwrap_or(false);

            if is_404 {
                None
            } else {
                Some(event)
            }
        })),
        ..Default::default()
    })
}
