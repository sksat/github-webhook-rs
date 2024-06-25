use std::env;
use std::path::PathBuf;

use anyhow::Result;

#[derive(Default)]
pub struct Opt {
    pub version: Version,
    pub out_path_ts: OutPathTs,
}

pub struct Version(pub String);

impl Default for Version {
    fn default() -> Self {
        Self("main".to_string())
    }
}

pub struct OutPathTs(pub PathBuf);

impl Default for OutPathTs {
    fn default() -> Self {
        let mut path = PathBuf::from(env::var("OUT_DIR").unwrap());
        path.push("schema.d.ts");
        Self(path)
    }
}

pub fn download_dts(
    Opt {
        version: Version(branch),
        out_path_ts: OutPathTs(dts_file),
    }: Opt,
) -> Result<()> {
    // setup .d.ts file
    let repo = "octokit/webhooks";
    let url =
        format!("https://raw.githubusercontent.com/{repo}/{branch}/payload-types/schema.d.ts");

    let response = minreq::get(url).send()?;
    let body = response.as_str()?;
    std::fs::write(&dts_file, body)?;

    Ok(())
}
