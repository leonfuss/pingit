use std::{
    sync::atomic::Ordering,
    sync::{atomic::AtomicBool, Arc},
};

fn main() {
    let running = Arc::new(AtomicBool::new(true));

    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");


    let ping = pingit::Ping::new("google.com".to_string());

    println!("pingit is active with address: 'google.com'. Ctr-C to end process \n output can be found at ping_record.csv");
    while running.load(Ordering::SeqCst) {
        ping.ping(&pingit::csv_writer);
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}
