use axum::http::HeaderMap;
use jsonwebtoken::{decode, encode, DecodingKey, Validation, errors::Error as JwtError, EncodingKey, Header};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Claims {
    // Define your JWT claims here (e.g., user ID, username, etc.)
    username: String
}

// Validates JWT token
pub fn validate_token( header_map: &HeaderMap, secret: &str) -> Result<Claims, JwtError> {
    let authorization_header = header_map
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    let token = authorization_header.trim_start_matches("Bearer ");

    // validate the token
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|token_data| token_data.claims)
}

pub fn generate_token(username: String) -> String {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims { username };
    
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());

    encode(&Header::default(), &claims, &encoding_key)
        .expect("Failed to generate JWT token")
}

