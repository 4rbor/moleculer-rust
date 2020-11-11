use crate::transporters;
use transporters::main;

// #[cfg(feature = "nats")]
use nats;
impl main::Connection<nats::Connection> for main::Transporter {
    fn connect(url: String) -> Result<nats::Connection, std::io::Error> {
        println!("Connecting to nats transporter.");

        nats::connect(&url)
    }
}
