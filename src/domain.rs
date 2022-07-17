use serde::Serialize;

#[derive(Clone, Debug)]
pub struct VirtualHost {
    pub domain: String,
    pub port: i32
}

impl VirtualHost {
    pub fn to_string(&self) -> String {
        return String::from(format!("domain: {}, port: {}", self.domain, self.port));
    }
}

#[derive(Clone, Serialize)]
pub struct Site {
    #[serde(rename(serialize = "{#NAME}"))]
    pub name: String,
    #[serde(rename(serialize = "{#URL}"))]
    pub url: String,
}