#[macro_use]
extern crate clap;

use clap::{Arg, App};

/// The application name
pub const NAME: &str = "lexa";

pub const VERBOSITY: &str = "verbosity";

pub(crate) fn clap_app(tmp_dir: &str) -> clap::App {
    App::new(NAME)
        .version(crate_version!())
        .about("Toolkit for BWC camera")
        //.author("The bol.com unFTP team")
        //.template("{bin} ({version}) - \n\t{usage}")
        .arg(
            Arg::with_name(VERBOSITY)
                .short("v")
                .multiple(true)
                .help("verbosity level")
        )
        .arg(
            Arg::with_name("host")
                .short("H")
                .multiple(true)
                .help("camera address")
        )
}

fn main() {
    //println!("Hello, world!");
    let arg_matches = clap_app("").get_matches();
    //println!("{:?}", arg_matches);
}
