pub enum IpVersion {
    V6,
    V4
}

pub enum EnumIpAddress {
    V4(String),
    V4_1(u32, u32, u32, u32),
    V6(String)
}

pub struct IpAddress {
    pub version: IpVersion,
    pub address: String
}
