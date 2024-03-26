use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation, errors::Error};

use crate::model;


pub struct TokenAuthentication{
    secret_key:String,
    issued_at:i64,
    expired_at:i64
}
#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
struct TokenCliams<T> {
    user:Option<T>,
    iat: i64, 
    exp: i64, 
}

impl TokenAuthentication {
    
    pub fn init() -> Self {
        let now = Utc::now().timestamp();
        let expiration = now + 3600;
        TokenAuthentication { secret_key: "h3evGTWiSjG9rYDstJC4q0eVTTen8aa5Y3UT+Q8p".to_string(), issued_at: now, expired_at: expiration }
    }

    pub fn generate_token(&self, user_data:&model::user_model::UserData) -> String {
        let token_claim = TokenCliams { user: Some(user_data), iat: self.issued_at, exp: self.expired_at };

        let header = Header::new(jsonwebtoken::Algorithm::HS256);

        match encode(&header, &token_claim, &EncodingKey::from_secret(self.secret_key.as_ref())) {
            Ok(token) => token,
            Err(e) => e.to_string(),
        }
    }

    pub fn validate_token(&self, token:&str) -> Result<TokenData<serde_json::Value>, Error> {
        decode::<serde_json::Value>(&token, &DecodingKey::from_secret(&self.secret_key.as_ref()), &Validation::default())
    }
}