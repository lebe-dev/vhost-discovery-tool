pub mod domain {
    #[derive(Clone)]
    pub struct VirtualHost {
        pub domain: String,
        pub port: i32
    }

    impl VirtualHost {
        pub fn to_string(&self) -> String {
            return String::from(format!("domain: {}, port: {}", self.domain, self.port));
        }
    }
}
