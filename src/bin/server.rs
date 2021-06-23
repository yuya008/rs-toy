use structopt::StructOpt;
use toy::opt::Opt;
use toy::server;

fn main() {
    let opt = Opt::from_args();
    if let Err(err) = server::run(&opt) {
        eprintln!("{}", err);
    }
}
