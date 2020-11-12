#[cfg(feature = "nats")]
pub fn connect(url: String) -> Result<nats::Connection, std::io::Error> {
    println!("Connecting to nats.");

    nats::connect(&url)
}
