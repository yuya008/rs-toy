use rs_toy::opt::Opt;
use rs_toy::server;
use std::process;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    if let Err(err) = server::run(&opt) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
