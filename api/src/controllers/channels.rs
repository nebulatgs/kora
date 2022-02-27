use super::*;
use crate::{
    states::{channels::ChannelState, identity::IdentityState},
    structs::channels::{Channel, ChannelCreateRequest, ChannelGetRequest, ChannelStartRequest, ChannelJoinRequest}, realtime::Stage,
};

#[get("/channels/list")]
pub async fn list(
    channel_state: web::Data<ChannelState>,
) -> web::Either<web::Json<Vec<Channel>>, HttpResponse> {
    if let Ok(guard) = channel_state.channels.lock() {
        return Either::Left(web::Json(guard.clone().into_values().collect::<Vec<_>>()));
    }
    Either::Right(HttpResponse::InternalServerError().finish())
}

#[get("/channels/get")]
pub async fn get(
    channel_state: web::Data<ChannelState>,
    channel: web::Query<ChannelGetRequest>,
) -> web::Either<web::Json<Option<Channel>>, HttpResponse> {
    if let Ok(guard) = channel_state.channels.lock() {
        return Either::Left(web::Json(guard.get(&channel.id).cloned()));
    }
    Either::Right(HttpResponse::InternalServerError().finish())
}

#[post("/channels/create")]
pub async fn create(
    channel_state: web::Data<ChannelState>,
    identity_state: web::Data<IdentityState>,
    channel_request: web::Json<ChannelCreateRequest>,
) -> web::Either<web::Json<Channel>, HttpResponse> {
    if let Ok(identity_guard) = identity_state.private_identities.lock() {
        if let Some(identity) = identity_guard.get(&channel_request.identity_id) {
            if let Ok(mut channel_guard) = channel_state.channels.lock() {
                let channel = Channel::new(channel_request.0.name, identity.to_owned().into());
                let res = web::Json(channel.clone());
                channel_guard.insert(channel.id, channel);
                return Either::Left(res);
            }
        }
    }
    Either::Right(HttpResponse::InternalServerError().finish())
}

#[post("/channels/start")]
pub async fn start(
    channel_state: web::Data<ChannelState>,
    identity_state: web::Data<IdentityState>,
    channel_start_request: web::Json<ChannelStartRequest>,
) -> web::Either<web::Json<Stage>, HttpResponse> {
    if let Ok(private_identities) = identity_state.private_identities.lock() {
        if let Some(request_identity) = private_identities.get(&channel_start_request.identity_id) {
            if let Ok(mut channel_guard) = channel_state.channels.lock() {
                if let Some(channel) = channel_guard.get_mut(&channel_start_request.id) {
                    if channel.initiator.public_id == request_identity.public_id {
                        channel.open();
                        let res = Stage::new(channel.clone(), channel_start_request.sdp.clone()).await;
                        dbg!(&res);
                        if let Ok(stage) = res {
                            let res = web::Json(stage);
                            return Either::Left(res);
                        }
                    }
                }
            }
        }
    }
    Either::Right(HttpResponse::InternalServerError().finish())
}

#[post("/channels/join")]
pub async fn join(
    channel_state: web::Data<ChannelState>,
    identity_state: web::Data<IdentityState>,
    channel_join_request: web::Json<ChannelJoinRequest>,
) -> web::Either<web::Json<Channel>, HttpResponse> {
    if let Ok(private_identities) = identity_state.private_identities.lock() {
        if let Some(request_identity) = private_identities.get(&channel_join_request.identity_id) {
            if let Ok(mut channel_guard) = channel_state.channels.lock() {
                if let Some(channel) = channel_guard.get_mut(&channel_join_request.id) {
                    if channel.initiator.public_id == request_identity.public_id {
                        channel.open();
                        let res = web::Json(channel.to_owned());
                        return Either::Left(res);
                    }
                }
            }
        }
    }
    Either::Right(HttpResponse::InternalServerError().finish())
}
