use super::*;
use crate::{
    states::identity::IdentityState,
    structs::{
        identity::{IdentityCreateRequest, Identity},
    },
};

#[post("/identity/create")]
pub async fn create(
    identity_state: web::Data<IdentityState>,
    create_request: web::Json<IdentityCreateRequest>,
) -> impl Responder {
    if let Ok(mut guard) = identity_state.identities.lock() {
        let identity = Identity::new(create_request.0.name, create_request.0.public_key);
        let res =  HttpResponse::Ok().body(identity.id.to_string());
        guard.insert(identity.id, identity);
        return res;
    }
    HttpResponse::InternalServerError().finish()
}

// #[get("/identity/edit")]
// pub async fn edit(identity_state: web::Data<IdentityState>) -> impl Responder {
//     // if let Ok(guard) = identity_state.channels.lock() {
//     //     return HttpResponse::Ok().body(guard.len().to_string());
//     // }
//     HttpResponse::InternalServerError().finish()
// }

// #[post("/identity/current")]
// pub async fn current(
//     identity_state: web::Data<IdentityState>,
// ) -> web::Either<web::Json<Identity>, HttpResponse> {
//     // if let Ok(mut guard) = identity_state.channels.lock() {
//     //     let channel = Channel::new(channel_request.name.clone());
//     //     let res = web::Json(channel.clone());
//     //     guard.insert(channel.id, channel);
//     //     return Either::Left(res);
//     // }
//     Either::Right(HttpResponse::InternalServerError().finish())
// }
