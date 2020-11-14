pub const INDRACORE_GENESIS_HASH: &'static str =
    "0x1c37775896d0cb956fd15eba915fdd86f2d7038881f071bdd725e40ec70135df";
#[derive(Clone, Eq, PartialEq, Debug, Copy)]
pub enum Network {
    Indracore,
    Unknow,
}

impl Default for Network {
    fn default() -> Self {
        Network::Indracore
    }
}

impl From<&str> for Network {
    fn from(name: &str) -> Network {
        match name {
            "indracore" => Network::Indracore,
            _ => Network::Unknow,
        }
    }
}

impl From<u8> for Network {
    fn from(v: u8) -> Network {
        match v {
            0 => Network::Indracore,
            _ => Network::Unknow,
        }
    }
}

impl From<u64> for Network {
    fn from(v: u64) -> Network {
        match v {
            0 => Network::Indracore,
            _ => Network::Unknow,
        }
    }
}

impl From<Network> for &'static str {
    fn from(n: Network) -> &'static str {
        match n {
            Network::Indracore => "indracore",
            _ => "unknow",
        }
    }
}

impl From<Network> for String {
    fn from(n: Network) -> String {
        match n {
            Network::Indracore => "indracore".to_string(),
            _ => "unknow".to_string(),
        }
    }
}

impl Network {
    pub fn genesis_hash(&self) -> &'static str {
        match self {
            Network::Indracore => INDRACORE_GENESIS_HASH,
            _ => "",
        }
    }

    pub fn from_genesis_hash(hash: &str) -> Self {
        match hash {
            INDRACORE_GENESIS_HASH => Network::Indracore,
            _ => Network::Unknow,
        }
    }
}
