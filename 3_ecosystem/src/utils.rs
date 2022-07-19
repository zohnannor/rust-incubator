use std::{
    convert::Infallible,
    env,
    sync::{Arc, Mutex},
};

use actix_identity::Identity;
use actix_web::{error, FromRequest};
use argonautica::{Error, Hasher, Verifier};
use futures::future::{ok, Ready};

pub fn hash_password(password: String) -> Result<String, Error> {
    Hasher::new()
        .with_password(password)
        .with_secret_key(env::var("SECRET_KEY").expect("SECRET_KEY expected"))
        .hash()
}

pub fn verify_password(password: String, hash: &str) -> Result<bool, Error> {
    Verifier::new()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(env::var("SECRET_KEY").expect("SECRET_KEY expected"))
        .verify()
}

#[derive(Clone, Default)]
pub struct UserSession(pub Arc<Mutex<Option<uuid::Uuid>>>);

impl UserSession {
    pub fn login(&self, id: uuid::Uuid) -> bool {
        self.0.lock().unwrap().replace(id).is_none()
    }

    pub fn logout(&self) -> bool {
        self.0.lock().unwrap().take().is_some()
    }

    pub fn is_logged(&self) -> bool {
        self.0.lock().unwrap().is_some()
    }

    pub fn id(&self) -> actix_web::Result<uuid::Uuid> {
        self.0.lock().unwrap().as_ref().map_or_else(
            || Err(error::ErrorUnauthorized("unauthorized")),
            |id| Ok(*id),
        )
    }
}

impl FromRequest for UserSession {
    type Error = Infallible;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        tracing::debug!("trying to extract identity");
        if let Ok(identity) = Identity::from_request(req, payload).into_inner() {
            let id = uuid::Uuid::parse_str(&identity.id().unwrap()).unwrap();
            return ok(Self(Arc::new(Mutex::new(Some(id)))));
        }
        ok(Self(Arc::new(Mutex::new(None))))
    }
}
