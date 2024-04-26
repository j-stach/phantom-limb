

pub mod motor;
pub mod sensor;

pub static IP = "127.0.0.1".parse::<std::net::IpAddr>().expect("Parse localhost address");

