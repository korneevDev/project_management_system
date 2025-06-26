use actix_web::{post, web, HttpResponse};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use crate::repositories::{Repository, DbPool};
use bcrypt::{verify, hash, DEFAULT_COST};

use actix_web_httpauth::extractors::bearer::BearerAuth; // Для работы с Bearer токенами
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // email пользователя
    pub exp: usize,   // срок действия
    pub role: String, // роль пользователя
}

#[post("/auth/login")]
pub async fn login(
    auth_data: web::Json<AuthRequest>,
    repo: web::Data<Repository>,
) -> HttpResponse {
    let user = match repo.users.find_by_email(&auth_data.email) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().json("Invalid credentials"),
    };

    if !verify(&auth_data.password, &user.password_hash).unwrap_or(false) {
        return HttpResponse::Unauthorized().json("Invalid credentials");
    }

    let tokens = generate_tokens(&user.email, &user.role, &repo.auth.jwt_secret);
    
    if let Err(e) = repo.auth.create_session(user.id, &tokens.refresh_token) {
        return HttpResponse::InternalServerError().json(format!("Failed to create session: {}", e));
    }

    HttpResponse::Ok().json(AuthResponse {
        token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_in: repo.auth.jwt_expiry_hours * 3600,
    })
}

#[post("/auth/refresh")]
pub async fn refresh(
    auth: BearerAuth,
    repo: web::Data<Repository>,
) -> HttpResponse {
    let session = match repo.auth.find_session(auth.token()) {
        Some(s) => s,
        None => return HttpResponse::Unauthorized().json("Invalid refresh token"),
    };

    if session.expires_at < Utc::now().naive_utc() {
        return HttpResponse::Unauthorized().json("Refresh token expired");
    }

    let user = match repo.users.find_by_id(session.user_id) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().json("User not found"),
    };

    let tokens = generate_tokens(&user.email, &user.role, &repo.auth.jwt_secret);

    if let Err(e) = repo.auth.update_session(session.id, &tokens.refresh_token) {
        return HttpResponse::InternalServerError().json(format!("Failed to update session: {}", e));
    }

    HttpResponse::Ok().json(AuthResponse {
        token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_in: repo.auth.jwt_expiry_hours * 3600,
    })
}

#[post("/auth/logout")]
pub async fn logout(
    auth: BearerAuth,
    repo: web::Data<Repository>,
) -> HttpResponse {
    // Удаляем сессию из БД
    if let Err(e) = repo.auth.delete_session_by_token(auth.token()) {
        return HttpResponse::InternalServerError().json(format!("Failed to logout: {}", e));
    }

    HttpResponse::Ok().json(json!({"message": "Logged out successfully"}))
}

#[derive(Debug)]
struct Tokens {
    access_token: String,
    refresh_token: String,
}

fn generate_tokens(email: &str, role: &str, secret: &str) -> Tokens {
    let access_exp = (Utc::now() + Duration::hours(1)).timestamp() as usize;
    let refresh_exp = (Utc::now() + Duration::days(7)).timestamp() as usize;

    let access_claims = Claims {
        sub: email.to_string(),
        exp: access_exp,
        role: role.to_string(),
    };

    let refresh_claims = Claims {
        sub: email.to_string(),
        exp: refresh_exp,
        role: role.to_string(),
    };

    Tokens {
        access_token: encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(secret.as_ref()),
        ).unwrap(),
        refresh_token: encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(secret.as_ref()),
        ).unwrap(),
    }
}
