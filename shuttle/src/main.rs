// shuttle/src/main.rs

// dependencies
use pavex::server::Server;
use shuttle_pavex::PavexService;
use shuttle_runtime::SecretStore;
use std::env::set_var;

mod shuttle_pavex;

#[shuttle_runtime::main]
async fn pavex(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_pavex::ShuttlePavex {
    set_var(
        "APP_PROFILE",
        secrets.get("APP_PROFILE").unwrap_or("dev".to_string()),
    );

    let _ = dotenvy::dotenv();

    let server = Server::new();

    let shuttle_px = PavexService(server);

    Ok(shuttle_px)
}
