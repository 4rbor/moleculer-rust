use crate::transporters;

pub fn resolve(url: String) {
    let v: Vec<&str> = url.split("://").collect();
    match v[0] {
        "nats" => {
            #[cfg(not(feature = "nats"))]
            panic!("The 'nats' feature is not currently enabled.");

            transporters::nats::connect(url).expect("Could not connect to nats transporter");
        }
        _ => println!("Unrecognized transporter url '{}'", url),
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
