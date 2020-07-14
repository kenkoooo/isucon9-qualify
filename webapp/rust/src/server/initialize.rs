use crate::constants::{DEFAULT_PAYMENT_SERVICE_URL, DEFAULT_SHIPMENT_SERVICE_URL};

use crate::IsuResult;
use actix_web::{post, web};
use serde_json::Value;
use sqlx::MySqlPool;
use std::process::Command;

#[post("/initialize")]
pub async fn post_initialize(
    request: web::Json<Value>,
    pool: web::Data<MySqlPool>,
) -> IsuResult<web::Json<Value>> {
    if Command::new("../sql/init.sh").status()?.success() {
        eprintln!("Initialized!");
    } else {
        eprintln!("Initialization failed!");
    }

    let payment_service_url = request
        .get("payment_service_url")
        .map(|x| x.as_str().unwrap())
        .unwrap_or(DEFAULT_PAYMENT_SERVICE_URL);
    let shipment_service_url = request
        .get("shipment_service_url")
        .map(|x| x.as_str().unwrap())
        .unwrap_or(DEFAULT_SHIPMENT_SERVICE_URL);

    let mut tx = pool.begin().await?;
    sqlx::query(
        "INSERT INTO configs (name, val) VALUES (?, ?) ON DUPLICATE KEY UPDATE val = VALUES(val)",
    )
    .bind("payment_service_url")
    .bind(payment_service_url)
    .execute(&mut tx)
    .await?;
    sqlx::query(
        "INSERT INTO configs (name, val) VALUES (?, ?) ON DUPLICATE KEY UPDATE val = VALUES(val)",
    )
    .bind("shipment_service_url")
    .bind(shipment_service_url)
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(web::Json(serde_json::json!({
        "campaign": 0,
        "language": "Rust"
    })))
}
