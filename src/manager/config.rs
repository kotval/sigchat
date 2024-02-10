use url::{Host, Url};

use crate::ServiceEnvironment;

pub struct Config {
    host: Host,
    service_environment: ServiceEnvironment,
    url: Url,
}

impl Config {
    pub fn new(host: Host, service_environment: ServiceEnvironment) -> Self {
        let host_base = host.to_string();
        match service_environment {
            ServiceEnvironment::Live => Self {
                host,
                service_environment,
                url: Url::parse(&format!("https://chat.{}", host_base)).unwrap(),
            },
            ServiceEnvironment::Staging => Self {
                host,
                service_environment,
                url: Url::parse(&format!("https://chat.staging.{}", host_base)).unwrap(),
            },
        }
    }

    pub fn host(&self) -> &Host { &self.host }

    pub fn service_environment(&self) -> &ServiceEnvironment { &self.service_environment }

    pub fn url(&self) -> &Url { &self.url }
}
