use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;
use cargo_metadata::{CargoOpt, MetadataCommand};

use github_webhook_dts_downloader::download_dts;

use github_webhook_type_generator::dts2rs;

fn main() -> Result<()> {
    println!("cargo:rerun-if-env-changed=GITHUB_WEBHOOK_SCHEMA_DTS");

    let manifest_dir = env!("CARGO_MANIFEST_DIR").to_string();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let metadata = MetadataCommand::new()
        .manifest_path(manifest_dir + "/Cargo.toml")
        .features(CargoOpt::AllFeatures)
        .no_deps() // prevent generate lockfile
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

    let octokit_webhooks = pkg
        .metadata
        .get("octokit-webhooks")
        .expect("Could not get octokit-webhooks metadata");

    let octokit_ver = octokit_webhooks
        .get("version")
        .expect("Could not get octokit/webhooks version")
        .as_str()
        .unwrap()
        .to_string();

    let dts_file = env::var("GITHUB_WEBHOOK_SCHEMA_DTS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| out_dir.join("schema.d.ts"));

    if !dts_file.try_exists()? {
        download_dts(github_webhook_dts_downloader::Opt {
            version: github_webhook_dts_downloader::Version(octokit_ver),
            out_path_ts: github_webhook_dts_downloader::OutPathTs(dts_file.clone()),
        })?;
    }

    let rs = dts2rs(&dts_file);
    let rs_file = out_dir.join("types.rs");

    let mut writer = BufWriter::new(File::create(&rs_file)?);
    write!(writer, "{rs}")?;
    writer.into_inner()?;

    let output = Command::new("rustfmt").arg(rs_file).output()?;
    let status = output.status;
    if !status.success() {
        io::stderr().write_all(&output.stderr).unwrap();
        panic!("failed to execute rustfmt: {status}")
    }

    Ok(())
}
