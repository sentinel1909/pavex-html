use opendal::services::Fs;
use opendal::{Error, Operator};
use pavex::blueprint::Blueprint;
use pavex::cookie::ProcessorConfig;
use pavex::{f, t};
use std::borrow::Cow;

pub fn register(bp: &mut Blueprint) {
    bp.prebuilt(t!(self::AppConfig));
    bp.singleton(f!(self::AppConfig::cookie_config));
}

#[derive(serde::Deserialize, Debug, Clone)]
/// The configuration object holding all the values required
/// to configure the application.
pub struct AppConfig {
    pub local_storage: OpenDalConfig,
    #[serde(default)]
    pub cookie: ProcessorConfig,
}

// methods for the AppConfig type
impl AppConfig {
    pub fn local_storage_config(&self) -> &OpenDalConfig {
        &self.local_storage
    }

    pub fn cookie_config(&self) -> ProcessorConfig {
        self.cookie.clone()
    }
}

// struct type to represent local OpenDal storage configuration
#[derive(serde::Deserialize, Clone, Debug)]
pub struct OpenDalConfig {
    pub root: Cow<'static, str>,
}

// methods for the OpenDalConfig type
impl OpenDalConfig {
    pub fn build(&self) -> Result<Operator, Error> {
        let builder = Fs::default().root(self.root.as_ref());
        let op: Operator = Operator::new(builder)?.finish();

        Ok(op)
    }
}
