#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate loggerv;
extern crate jenkins_api;
extern crate chrono;
#[macro_use]
extern crate failure;
extern crate libc;
extern crate openssl_probe;

pub mod jenkins;
pub use jenkins::{Result, Error};

#[allow(unused_imports)]
use std::collections::BTreeMap;
use clap::{Arg, App, AppSettings, SubCommand, ArgMatches};
use std::process;

fn print_error_debug(e: &Error) {
    for cause in e.iter_chain().skip(1) {
        warn!("caused by: {}", cause);
        //warn!("{:?}", cause); // debug for error-chain traces etc
    }
}

fn has_equals(v: String) -> std::result::Result<(), String> {
    if !v.contains(":") {
        return Err(String::from("Must be a key:value pair"))
    }
    Ok(())
}

fn main() {
    let app = App::new("jenq")
        .version(crate_version!())
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .global_settings(&[AppSettings::ColoredHelp])
        .about("query jenkins for job results")
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Increase verbosity"))
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Adds line numbers to log statements"))
        .arg(Arg::with_name("job-name")
            .required(true)
            .help("Job name"))
        .arg(Arg::with_name("filter")
            .multiple(true)
            .short("f")
            .long("filter")
            .takes_value(true)
            .value_terminator(":")
            .validator(has_equals)
            .help("Filter on a set of key:values in jenkins build parameters"))
        .subcommand(SubCommand::with_name("console")
            .arg(Arg::with_name("number")
                .help("Build number if not last"))
            .about("Print the latest jenkins console text for a service deploy"))
        .subcommand(SubCommand::with_name("history")
            .about("Print the jenkins deployment history for a service"))
        .subcommand(SubCommand::with_name("latest")
            .about("Print the latest jenkins deployment job for a service"));

    let args = app.get_matches();
    let name = args.subcommand_name().unwrap();
    let _ = run(&args).map_err(|e| {
        error!("{} error: {}", name, e);
        print_error_debug(&e);
        process::exit(1);
    });
    process::exit(0);
}

fn run(args: &ArgMatches) -> Result<()> {
    // initialise deps and set log default - always show INFO messages (+1)
    loggerv::Logger::new()
        .verbosity(args.occurrences_of("verbose") + 1)
        .module_path(true) // may need cargo clean's if it fails..
        .line_numbers(args.is_present("debug"))
        .init()
        .unwrap();

    openssl_probe::init_ssl_cert_env_vars();
    // Ignore SIGPIPE errors to avoid having to use let _ = write! everywhere
    // See https://github.com/rust-lang/rust/issues/46016
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    // Dispatch arguments to internal handlers. Pass on handled result.
    dispatch_commands(args)
}

fn dispatch_commands(a: &ArgMatches) -> Result<()> {
    let job = a.value_of("job-name").unwrap();

    let mut params = BTreeMap::new();
    if let Some(vals) = a.values_of("filter") {
        for x in vals {
            let pair = x.split(":").collect::<Vec<_>>();
            params.insert(pair[0].to_string(), pair[1].to_string());
        }
    }
    debug!("Using filters: {:?}", params);

    return if let Some(_) = a.subcommand_matches("latest") {
        jenkins::latest_build(&job, &params)
    }
    else if let Some(b) = a.subcommand_matches("console") {
        if let Some(n) = b.value_of("number") {
            let nr : u32 = n.parse().unwrap();
            jenkins::specific_console(&job, nr, &params)
        } else {
            jenkins::latest_console(&job, &params)
        }
    }
    else if let Some(_) = a.subcommand_matches("history") {
       jenkins::history(&job, &params)
    } else {
        unreachable!()
    };
}
