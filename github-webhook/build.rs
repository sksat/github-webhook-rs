use anyhow::Result;
use cargo_metadata::{CargoOpt, MetadataCommand};
use std::env;

use github_webhook_dts_downloader::run_transform;

fn main() -> Result<()> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR").to_string();
    let metadata = MetadataCommand::new()
        .manifest_path(manifest_dir + "/Cargo.toml")
        .features(CargoOpt::AllFeatures)
        .exec()
        .unwrap();

    // workspace manifest
    assert_ne!(metadata.workspace_members.len(), 0);

    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg = &metadata
        .packages
        .iter()
        .find(|p| p.name == pkg_name)
        .unwrap();

    assert_eq!(pkg_name, pkg.name);

    run_transform(Default::default())?;

    Ok(())
}
