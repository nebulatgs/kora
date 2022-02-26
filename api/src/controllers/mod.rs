pub(super) use actix_web::{
    get,
    http::header,
    post,
    web::{self, Either},
    HttpRequest, HttpResponse, Responder,
};
pub(super) use uuid::Uuid;

use crate::structs::identity::Identity;

pub(super) async fn verify_request(request: HttpRequest, body: web::Bytes, identity: &Identity) -> bool {
    if let Some(signature) = request.headers().get("Signature") {
        if let Ok(signature) = signature.to_str() {
            if let Ok(signature) = serde_json::from_str::<ed25519_dalek::Signature>(signature) {
                dbg!(signature, &body);
                use ed25519_dalek::Verifier;
                    return identity.public_key.verify(&body, &signature).is_ok();
            }
        }
    }
    false
}

pub mod channels;
pub mod identity;
pub mod index;
pub mod meta;
