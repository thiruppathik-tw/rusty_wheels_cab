mod ecu_server;
mod occupancy;

fn main() {
    println!("Starting server.");
    let _r = ecu_server::server_init();
}
