use std::fs::{File, OpenOptions};

use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct PingRecord {
    ping_address: String,
    response_latency: f64,
    dropped_packages: String,
    timestamp: String,
}

pub struct Ping(String);

impl Ping {
    pub fn new(ping_address: String) -> Ping {
        Ping(ping_address)
    }

    pub fn ping(&self, writer: &dyn Fn(PingRecord)) {
        let mut ping = oping::Ping::new();

        ping.set_timeout(5.0).expect("Failed to set timeout");
        ping.add_host("1.1.1.1").expect("invalid ping address");

        let responses = ping
            .send()
            .expect("Please run command with sudo permission");

        for response in responses {
            let record = PingRecord {
                ping_address: Self::create_address(&self, response.address),
                dropped_packages: response.dropped.to_string(),
                response_latency: response.latency_ms,
                timestamp: chrono::Local::now().to_string(),
            };

            writer(record);
        }
    }
    fn create_address(&self, address: String) -> String {
        format!("{}<{}>", self.0, address)
    }
}

pub fn csv_writer(record: PingRecord) {
    let writer = OpenOptions::new()
        .write(true)
        .append(true)
        .open("ping_record.csv")
        .expect("Error creating file");

    let mut wrt = csv::Writer::from_writer(writer);

    wrt.serialize(record)
        .expect("Failed to write record to csv file");
    wrt.flush()
        .expect("Someting went wrong while writing to csv file");
}
