use crate::{
    telemetry::{segment::Batch, SegmentClient},
    Arguments, CommandError, CommandSuccess,
};
use anyhow::{anyhow, Result};
use clap::Args;
use qlty_analysis::version::BUILD_PROFILE;
use qlty_config::Workspace;
use std::path::PathBuf;
use tracing::{debug, info};

const SENTRY_DSN: Option<&str> = option_env!("SENTRY_DSN");

#[derive(Args, Debug)]
pub struct Telemetry {
    #[arg(long)]
    pub segment_payload: Option<PathBuf>,

    #[arg(long)]
    pub sentry_payload: Option<PathBuf>,
}

impl Telemetry {
    pub fn execute(&self, _args: &Arguments) -> Result<CommandSuccess, CommandError> {
        if self.segment_payload.is_some() {
            return self.send_payload_to_segment();
        }

        if self.sentry_payload.is_some() {
            return self.send_payload_to_sentry();
        }

        CommandSuccess::ok()
    }

    fn send_payload_to_segment(&self) -> Result<CommandSuccess, CommandError> {
        let payload_path = self.segment_payload.clone().unwrap();
        let current = std::env::current_dir().expect("current dir");
        let repository_path = Workspace::closest_git_repository_path(&current);

        let payload = std::fs::read_to_string(&payload_path).map_err(|err| {
            anyhow!(
                "Unable to read telemetry payload file: {} ({:?})",
                payload_path.display(),
                err
            )
        })?;

        let client = SegmentClient::new(repository_path.clone())?;
        let batch: Batch = serde_json::from_str(&payload).unwrap();
        client.send_batch(batch)?;

        CommandSuccess::ok()
    }

    fn send_payload_to_sentry(&self) -> Result<CommandSuccess, CommandError> {
        let dsn = SENTRY_DSN.unwrap_or_default();
        if dsn.is_empty() {
            // ignore telemetry if no DSN is set
            debug!("No Sentry DSN set, skipping telemetry");
            return CommandSuccess::ok();
        }

        let payload_path = self.sentry_payload.clone().unwrap();
        let payload = std::fs::read_to_string(&payload_path).map_err(|err| {
            anyhow!(
                "Unable to read telemetry payload file: {} ({:?})",
                payload_path.display(),
                err
            )
        })?;

        let event: ::sentry::protocol::Event = serde_json::from_str(&payload).unwrap();

        let _guard = sentry::init((
            dsn.to_string(),
            sentry::ClientOptions {
                debug: true,
                release: sentry::release_name!(),
                environment: match BUILD_PROFILE {
                    "release" => Some("production".into()),
                    _ => Some("development".into()),
                },
                default_integrations: false,
                ..Default::default()
            },
        ));

        sentry::capture_event(event);

        info!(
            "Telemetry payload sent to Sentry: {}",
            payload_path.display()
        );

        CommandSuccess::ok()
    }
}
