use anyhow::Result;

use github_webhook_dts_downloader::run_transform;

fn main() -> Result<()> {
    run_transform(Default::default())?;

    Ok(())
}
