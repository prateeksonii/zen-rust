use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    db::get_db_pool,
    models::{auth::User, common::ResBody},
};

#[derive(Deserialize)]
struct SignupDto {
    name: String,
    username: String,
    email: String,
    password: String,
}

async fn sign_up(Json(payload): Json<SignupDto>) -> Response {
    let pool = get_db_pool().await.expect("pool error");

    let row: Option<User> =
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", payload.email)
            .fetch_optional(&pool)
            .await
            .unwrap();

    if let Some(_) = row {
        return (
            StatusCode::CONFLICT,
            Json(ResBody {
                ok: false,
                result: "User exists",
            }),
        )
            .into_response();
    }

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash_result = argon2.hash_password(payload.password.as_bytes(), &salt);
    if let Ok(password_hash) = password_hash_result {
        let inserted_row_result = sqlx::query_as!(
            User,
            "INSERT INTO users (
                                name,
                                username,
                                email,
                                password
                            ) VALUES (
                                $1,
                                $2,
                                $3,
                                $4
                            ) RETURNING *
                            ",
            payload.name,
            payload.username,
            payload.email,
            password_hash.to_string(),
        )
        .fetch_one(&pool)
        .await;

        if let Ok(inserted_user) = inserted_row_result {
            println!("{}", &inserted_user.email);
            (
                StatusCode::CREATED,
                Json(ResBody {
                    ok: true,
                    result: inserted_user,
                }),
            )
                .into_response()
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "ok": false
                })),
            )
                .into_response()
        }
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "ok": false
            })),
        )
            .into_response()
    }
}

pub fn auth_routes() -> Router {
    let router = Router::new().route("/sign-up", post(sign_up));

    router
}
