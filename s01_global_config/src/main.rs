use std::thread::spawn;

use s01_global_config::config1;
use s01_global_config::config2;
use s01_global_config::init;
fn main() {
    init();

    spawn(|| {
        println!("method1 {:#?}", *config1::CONFIG);
        println!("method1 {:#?}", *config1::Config::global());
        println!("method2 {:#?}", *config2::Config::global());
    }).join().unwrap();
}
