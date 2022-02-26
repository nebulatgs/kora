use super::*;
use crate::{
    states::{channels::ChannelState, identity::IdentityState},
    structs::channels::{Channel, ChannelCreateRequest},
};

#[get("/channels/list")]
pub async fn list(channel_state: web::Data<ChannelState>) -> impl Responder {
    if let Ok(guard) = channel_state.channels.lock() {
        return HttpResponse::Ok().body(guard.len().to_string());
    }
    HttpResponse::InternalServerError().finish()
}

#[post("/channels/create")]
pub async fn create(
    body: web::Bytes,
    channel_state: web::Data<ChannelState>,
    identity_state: web::Data<IdentityState>,
    channel_request: web::Json<ChannelCreateRequest>,
) -> web::Either<web::Json<Channel>, HttpResponse> {
    dbg!(&body);
    if let Ok(identity_guard) = identity_state.identities.lock() {
        if let Some(identity) = identity_guard.get(&channel_request.identity_id) {
            if let Ok(mut channel_guard) = channel_state.channels.lock() {
                let channel = Channel::new(channel_request.0.name, identity.to_owned());
                let res = web::Json(channel.clone());
                channel_guard.insert(channel.id, channel);
                return Either::Left(res);
            }
        }
    }
    Either::Right(HttpResponse::InternalServerError().finish())
}
