use std::io;

use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Config<'a> {
    pub addr: &'a str, //最好不要用引用
    pub port: u32,
}
pub static INSTANCE: OnceCell<Config> = OnceCell::new();

impl Config<'_> {
    pub fn global() -> &'static Config<'static> {
        INSTANCE.get().expect("Config is not initialized")
    }
}

pub fn init(addr: &'_ str, port: u32) -> Result<(), io::Error> {
    let conf = Config{addr:"", port};
    unsafe{
        let p = std::ptr::addr_of!(conf.addr) as *mut &str;
        std::ptr::write(p, addr);
    }
    INSTANCE.set(conf).unwrap();
    Ok(())
}