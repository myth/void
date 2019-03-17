extern crate ansi_term;
extern crate clap;
#[macro_use]
extern crate log;
extern crate walkdir;

mod io;
mod logging;

use clap::{crate_authors, crate_version, App, Arg};
use log::LevelFilter;

use io::VoidWalk;

const ABOUT: &str = "
Very Original Interface Definition (void) generates source code
in different languages from constructs defined in .void files.

The void language consists of defining versioned modules and APIs
containing types, object definitions and endpoints. APIs are
a special type of module that exports the types, objects and
endpoints of the imported modules, which is used to create an
abstract syntax tree (AST).

The AST is then used by different backends to generate source
code in the desired target languages, with serialization
and deserialization built in. Additional server and client
code is created that handles versioning, serialization,
validation and basic error handling.
";

fn main() {
    let matches = App::new("Very Original Interface Definition (void)")
        .version(crate_version!())
        .author(crate_authors!())
        .about(ABOUT)
        .help_message("Prints help information. Use --help for more details.")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity in the compiler output."),
        )
        .arg(
            Arg::with_name("INPUT_DIR")
                .help("The root directory to scan for .void files.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT_DIR")
                .help("The output directory to store generated source code.")
                .required(true)
                .index(2),
        )
        .get_matches();

    let log_level = match matches.occurrences_of("v") {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 | _ => LevelFilter::Trace,
    };

    logging::init(log_level).unwrap_or_else(|e| {
        eprintln!("Failed to initialize logging: {}", e);
        std::process::exit(3);
    });

    let voidwalk = VoidWalk::new(matches.value_of("INPUT_DIR").unwrap());
    voidwalk.read_files();
}
