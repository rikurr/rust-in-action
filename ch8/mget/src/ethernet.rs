use rand::{self, RngCore};
#[cfg(target_os = "linux")]
use smoltcp::write;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octet = self.0;
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octet[0], octet[1], octet[2], octet[3], octet[4], octet[5]
        )
    }
}

impl MacAddress {
    pub fn new() -> Self {
        let mut octets: [u8; 6] = [0; 6];
        rand::thread_rng().fill_bytes(&mut octets);
        octets[0] |= 0b_0000_0010;
        octets[0] &= 0b_1111_1110;
        Self(octets)
    }
}

impl Into<write::EthernetAddress> for MacAddress {
    fn into(self) -> write::EthernetAddress {
        write::EthernetAddress { 0: self.0 }
    }
}
