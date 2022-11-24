pub mod config1;
pub mod config2;

pub fn init(){
    config1::init(&"127.0.0.1", 8080).unwrap();
    config2::init(&"127.0.0.1", 8080).unwrap();
}