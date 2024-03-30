use log;
use log4rs;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default());
    let who = "world";
    let n = 5;
    log::info!("who:{},n:{}", who, n);
}