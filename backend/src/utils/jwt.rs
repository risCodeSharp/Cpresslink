use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};


// TODO: LATER Subscription add in the auth
#[derive(Serialize, Deserialize,Clone)]
pub struct JwtClaims {
    pub sub: i32,
    pub exp: usize,
}

pub fn generate_token(user_id: i32, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_claims = JwtClaims {
        sub: user_id,
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };
    encode(
        &Header::default(),
        &jwt_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn verify_token(token: &str, secret: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::default();

    // Ensure the token is not expired
    validation.validate_exp = true;

    // No extra grace period for expored tokens (strict timing)
    validation.leeway = 0;

    decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
}
