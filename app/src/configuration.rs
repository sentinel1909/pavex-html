use opendal::services::Fs;
use opendal::{Error, Operator};
use pavex::blueprint::Blueprint;
use pavex::t;
use std::borrow::Cow;

pub fn register(bp: &mut Blueprint) {
    bp.prebuilt(t!(self::AppConfig));
}

#[derive(serde::Deserialize, Debug, Clone)]
/// The configuration object holding all the values required
/// to configure the application.
pub struct AppConfig {
    pub local_storage: OpenDalConfig,
}

// methods for the AppConfig type
impl AppConfig {
    pub fn local_storage_config(&self) -> &OpenDalConfig {
        &self.local_storage
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
