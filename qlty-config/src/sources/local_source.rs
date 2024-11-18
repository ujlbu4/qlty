use super::{source::SourceFetch, Source};
use crate::Library;
use anyhow::Result;
use std::path::PathBuf;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct LocalSource {
    pub library: Library,
    pub origin: PathBuf,
}

impl Source for LocalSource {
    fn local_root(&self) -> PathBuf {
        self.origin.clone()
    }

    fn clone_box(&self) -> Box<dyn Source> {
        Box::new(self.clone())
    }
}

impl SourceFetch for LocalSource {
    fn fetch(&self) -> Result<()> {
        debug!("Skipping source fetch: {:?}", self.origin);
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn SourceFetch> {
        Box::new(self.clone())
    }
}
