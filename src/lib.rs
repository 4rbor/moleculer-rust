use crate::transporters::main;

mod transporters {
    pub mod main;
    pub mod nats;
}

pub struct ServiceBroker {
    pub namespace: Option<String>,
    pub transporter: String,
    started: Option<bool>,
}

impl ServiceBroker {
    pub fn new(transporter: String) -> ServiceBroker {
        ServiceBroker {
            namespace: None,
            transporter: transporter,
            started: None,
        }
    }

    pub fn start(&mut self) {
        println!("Starting service broker.");
        self.started = Some(true);

        main::resolve(self.transporter.to_string());
    }

    pub fn stop(&mut self) {
        println!("Starting service broker.");
        self.started = Some(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let transporter = String::from("nats://blah:4222");
        let mut broker2 = ServiceBroker::new(transporter);
        broker2.start();
    }
}
