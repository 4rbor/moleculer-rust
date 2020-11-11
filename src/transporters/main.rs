use crate::transporters::nats;

pub struct Transporter {
    // connect: dyn MyTrait,
}
pub trait Connection<T> {
    fn connect(url: String) -> std::io::Result<T>;
}

pub fn resolve(url: String) {
    let v: Vec<&str> = url.split("://").collect();
    match v[0] {
        // TODO: Left off here, need a way to instantiate rather than
        // trying to directly connect

        // "nats" => nats::connect(url.to_string()),
        _ => println!("Unrecognized transporter url '{}'", url.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn connection_result() {
        resolve("nats://localhost:4222".to_string());
    }
}
