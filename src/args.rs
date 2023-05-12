use clap::{App, Arg};

pub struct Args {
    pub memory: Option<i32>,
    pub frequency: Option<i32>,
}

pub fn parse_and_validate_args() -> Args {
    let matches = App::new("oracle_shield")
        .arg(
            Arg::new("memory")
                .short('m')
                .long("memory")
                .value_name("MEMORY")
                .help("Sets the memory in GiB")
                .takes_value(true),
        )
        .arg(
            Arg::new("frequency")
                .short('f')
                .long("frequency")
                .value_name("FREQUENCY")
                .help("Sets the frequency of calculation per month")
                .takes_value(true),
        )
        .get_matches();

    let memory = matches
        .value_of("memory")
        .map(|s| s.parse::<i32>().expect("Invalid value for memory argument"));

    let frequency = matches
        .value_of("frequency")
        .map(|s| s.parse::<i32>().expect("Invalid value for frequency argument"));

    Args { memory, frequency }
}
