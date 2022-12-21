use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use anyhow::Result;

use github_webhook_type_generator::dts2rs;

fn main() -> Result<()> {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let pkg_version = env::var("CARGO_PKG_VERSION").unwrap();
    let pkg_version: Vec<&str> = pkg_version.split("+").collect();
    let version = if pkg_version.len() == 2 {
        pkg_version[1]
    } else {
        "master"
    };

    // setup .d.ts file
    let repo = "octokit/webhooks";
    let url =
        format!("https://raw.githubusercontent.com/{repo}/{version}/payload-types/schema.d.ts");

    let body = reqwest::blocking::get(&url)?.text()?;
    let dts_file = out_path.join("schema.d.ts");
    std::fs::write(&dts_file, body)?;

    let rs = dts2rs(dts_file.to_str().unwrap()).unwrap();

    let mut writer = BufWriter::new(File::create(format!("{}/types.rs", out_path.display()))?);
    write!(writer, "{}", rs)?;

    Ok(())
}
