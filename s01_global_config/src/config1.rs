#[derive(Debug)]
pub struct Config<'a> {
    pub addr: &'a str,
    pub port: u32,
}

lazy_static::lazy_static! {
    pub static ref CONFIG: Config<'static> = Config{
        addr: "",
        port: 0,
    };
}

impl Config<'_> {
    pub fn global() -> &'static Config<'static> {
        &*CONFIG
    }
}

pub fn init(addr: &str, port: u32) -> Result<(), std::io::Error> {
    let p = std::ptr::addr_of!(*CONFIG) as *mut Config;
    unsafe{
        std::ptr::write(p, Config{ addr, port});
    }
    Ok(())
}