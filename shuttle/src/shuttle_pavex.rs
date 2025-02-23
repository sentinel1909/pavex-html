// shuttle/src/shuttle_pavex.rs

// dependencies
use pavex::server::Server;
use server::configuration::Config;
use server_sdk::{build_application_state, run};
use shuttle_runtime::{CustomError, Error, Service};
use std::net::SocketAddr;

// type declarations
pub type ShuttlePavex = Result<PavexService, Error>;

// A wrapper type for a Pavex Server, so we can implement shuttle_runtime::Service for it.
pub struct PavexService(pub Server);

#[shuttle_runtime::async_trait]
impl Service for PavexService {
    async fn bind(mut self, addr: SocketAddr) -> Result<(), Error> {
        let config = Config::load(None)?;
        let application_state = build_application_state(config.app).await;

        let server = self
            .0
            .bind(addr)
            .await
            .expect("Failed to bind the server TCP listener");

        tracing::info!("Starting to listen for incoming requests at {}", addr);

        run(server, application_state.map_err(|e| {CustomError::new(e)})?).await;

        Ok(())
    }
}

impl From<Server> for PavexService {
    fn from(router: Server) -> Self {
        Self(router)
    }
}
