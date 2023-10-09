use std::fmt::{Display, Formatter};

use serde::Serialize;

#[derive(Clone, Debug)]
pub struct VirtualHost {
    pub domain: String,
    pub port: i32
}

impl Display for VirtualHost {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "domain: '{}', port: {}", self.domain, self.port)
    }
}

#[derive(Clone, Serialize)]
pub struct Site {
    #[serde(rename(serialize = "{#NAME}"))]
    pub name: String,
    #[serde(rename(serialize = "{#URL}"))]
    pub url: String,
}