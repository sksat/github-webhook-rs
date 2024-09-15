use std::path::PathBuf;

use clap::Parser;

use github_webhook_type_generator::*;

#[derive(Debug, Parser)]
#[command(version)]
struct Opt {
    #[arg()]
    dts_file: PathBuf,
}

fn main() {
    let opt = Opt::parse();

    let rs = dts2rs(&opt.dts_file);
    print!("{}", rs);
}
