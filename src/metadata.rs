use std::borrow::Cow;

use camino::Utf8Path;
use cargo_metadata::{Metadata, MetadataCommand, Package};
use lazy_static::lazy_static;

use crate::path::PathExt;

pub(crate) fn metadata() -> &'static Metadata {
    lazy_static! {
        static ref METADATA: Metadata = MetadataCommand::new()
            .no_deps()
            .other_options(["--offline".to_string()])
            .exec()
            // TODO: Error handling
            .unwrap();
    }

    &METADATA
}

pub(crate) trait MetadataExt {
    fn target_dir(&self) -> Cow<Utf8Path>;
    fn uniffi_crates(&self) -> Vec<&Package>;
}

impl MetadataExt for Metadata {
    fn target_dir(&self) -> Cow<Utf8Path> {
        let target_dir = self.target_directory.as_path();
        let relative = target_dir.to_relative();

        match relative {
            Ok(relative) => Cow::from(relative),
            Err(_) => Cow::from(target_dir),
        }
    }

    /// Returns the package metadata for all crates that depend on UniFFI and are below, at or above the current working directory.
    fn uniffi_crates(&self) -> Vec<&Package> {
        let cwd = std::env::current_dir().unwrap();
        // TODO: also include crates that are above the current working directory
        let crates: Vec<_> = self
            .workspace_packages()
            .into_iter()
            .filter(|package| package.manifest_path.starts_with(&cwd))
            .collect();

        crates
    }
}
