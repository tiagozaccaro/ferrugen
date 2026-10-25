use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use application::{ApplicationError, TenantService};
use contracts::health;
use persistence::persistence::{MigrationState, PgPool};
use std::time::Duration;

pub async fn health_live() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn is_ready(pool: &PgPool) -> bool {
    pool.ping().await.is_ok() && migrations_applied(pool)
}

fn migrations_applied(pool: &PgPool) -> bool {
    pool.query_one::<bool>("SELECT EXISTS(SELECT 1 FROM migration_state WHERE name = 'applied')")
        .await
        .map(|row| row.get(0))
        .unwrap_or(false)
}

pub async fn health_ready(ready: web::Data<AppState>, pool: web::Data<PgPool>) -> impl Responder {
    let ready = ready.clone();
    let db = ready.db.as_ref().cloned();
    let pool = pool.inner().clone();
    match is_ready(&pool) {
        true => HttpResponse::Ok().finish(),
        false => HttpResponse::ServiceUnavailable().body(application_error_to_text(db)),
    }
}

fn application_error_to_text(_e: Option<ApplicationError>) -> String {
    "not_ready_migrations_failed".to_string()
}

pub fn configure_app(cfg: &mut web::Configurator<HttpServer>) {
    cfg.service(
        web::scope("health")
            .route("", web::route().get(health_live))
            .route("/ready", web::route().get(health_ready)),
    );
}

#[actix_web::main]
async fn main() {
    // Placeholder main loop; real impl wires config + pool per contract.
}
