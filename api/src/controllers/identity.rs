use super::*;
use crate::{
    states::identity::IdentityState,
    structs::{
        identity::{IdentityCreateRequest, PrivateIdentity},
    },
};

#[post("/identity/create")]
pub async fn create(
    identity_state: web::Data<IdentityState>,
    create_request: web::Json<IdentityCreateRequest>,
) -> impl Responder {
    if let Ok(mut guard) = identity_state.private_identities.lock() {
        let identity = PrivateIdentity::new(create_request.0.name);
        let res =  HttpResponse::Ok().body(identity.private_id.to_string());
        guard.insert(identity.private_id, identity);
        return res;
    }
    HttpResponse::InternalServerError().finish()
}
