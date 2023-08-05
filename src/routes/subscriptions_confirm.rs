use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}
#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters, pool))]
pub async fn confirm(parameters: web::Query<Parameters>, pool: web::Data<PgPool>) -> HttpResponse {
    let subscription_token = &parameters.subscription_token;

    match update_subscription_status_to_confirmed(&pool, subscription_token).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

#[tracing::instrument(
    name = "Updating subscription status to confirmed in database",
    skip(pool, token)
)]
async fn update_subscription_status_to_confirmed(
    pool: &PgPool,
    token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            UPDATE subscriptions 
            SET status = 'confirmed' 
            WHERE id = (
                SELECT subscriber_id FROM subscription_tokens WHERE subscription_token = ($1)
            )
        "#,
        token,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
