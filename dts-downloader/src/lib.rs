use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;

use github_webhook_type_generator::dts2rs;

#[derive(Default)]
pub struct Opt {
    pub version: Version,
    pub out_path_ts: OutPathTs,
    pub out_path_rs: OutPathRs,
}

pub struct Version(pub String);

impl Default for Version {
    fn default() -> Self {
        let pkg_version = env::var("CARGO_PKG_VERSION").unwrap();
        let splitted: Vec<&str> = pkg_version.split('+').collect();
        let branch_name = if splitted.len() == 2 {
            splitted[1]
        } else {
            "master"
        }
        .to_string();
        Self(branch_name)
    }
}

pub struct OutPathTs(PathBuf);

impl Default for OutPathTs {
    fn default() -> Self {
        let mut path = PathBuf::from(env::var("OUT_DIR").unwrap());
        path.push("schema.d.ts");
        Self(path)
    }
}

pub struct OutPathRs(PathBuf);

impl Default for OutPathRs {
    fn default() -> Self {
        let mut path = PathBuf::from(env::var("OUT_DIR").unwrap());
        path.push("types.rs");
        Self(path)
    }
}

pub fn run_transform(
    Opt {
        version: Version(branch),
        out_path_ts: OutPathTs(dts_file),
        out_path_rs: OutPathRs(rs_file),
    }: Opt,
) -> Result<()> {
    // setup .d.ts file
    let repo = "octokit/webhooks";
    let url =
        format!("https://raw.githubusercontent.com/{repo}/{branch}/payload-types/schema.d.ts");

    let body = reqwest::blocking::get(url)?.text()?;
    std::fs::write(&dts_file, body)?;

    let rs = dts2rs(&dts_file);

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
