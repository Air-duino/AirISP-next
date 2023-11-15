use clap::ArgMatches;

pub struct AirISP {
    port: String,
    baud: u32,
    trace: bool,
    connect_attempts: u32,
    before: String,
    after: String,
    peripheral: String,
}

impl AirISP
{
    pub fn new(matches:ArgMatches) -> AirISP
    {
        AirISP {
            port: matches.get_one::<String>("port").unwrap().to_string(),
            baud: *matches.get_one::<u32>("baud").unwrap(),
            trace: *matches.get_one::<bool>("trace").unwrap(),
            connect_attempts: *matches.get_one::<u32>("connect_attempts").unwrap(),
            before: matches.get_one::<String>("before").unwrap().to_string(),
            after: matches.get_one::<String>("after").unwrap().to_string(),
            peripheral: matches.get_one::<String>("peripheral").unwrap().to_string(),
        }
    }
}