use std::path::PathBuf;

use structopt::StructOpt;

use github_webhook_type_generator::*;

#[derive(Debug, StructOpt)]
struct Opt {
    dts_file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let rs = dts2rs(&opt.dts_file);
    print!("{}", rs);
}
