
extern crate clap;
use clap::{crate_version, crate_description, Arg, App, ArgMatches};


fn is_int(v: String) -> Result<(), String> {
    match v.parse::<u16>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("value must be valid positive integer"))
    }
}

pub fn cli() -> ArgMatches<'static> {
    App::new("mdpreview")
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("port")
              .short("p")
              .long("port")
              .takes_value(true)
              .default_value("4000")
              .validator(is_int)
              .help("Port number to bind preview server to")
        )
        .get_matches()
}

