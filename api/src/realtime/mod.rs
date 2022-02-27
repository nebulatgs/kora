use std::{sync::Arc, time::Duration};

use anyhow::Result;
use serde::Serialize;
use tokio::sync::mpsc::Receiver;
use webrtc::{
    api::{
        interceptor_registry::register_default_interceptors, media_engine::MediaEngine, APIBuilder,
    },
    ice_transport::ice_server::RTCIceServer,
    interceptor::registry::Registry,
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription,
    },
    rtcp::payload_feedbacks::picture_loss_indication::PictureLossIndication,
    rtp_transceiver::{rtp_codec::RTPCodecType, rtp_receiver::RTCRtpReceiver},
    track::{
        track_local::{track_local_static_rtp::TrackLocalStaticRTP, TrackLocal, TrackLocalWriter},
        track_remote::TrackRemote,
    },
    Error,
};

use crate::structs::channels::Channel;
#[derive(Serialize, Debug)]
pub struct Stage {
    channel: Channel,
    sdp: RTCSessionDescription,
    #[serde(skip_serializing)]
    local_track_chan_rx: Receiver<Arc<TrackLocalStaticRTP>>,
}

impl Stage {
    pub async fn new(channel: Channel, sdp: RTCSessionDescription) -> Result<Self> {
        let mut engine = MediaEngine::default();
        engine.register_default_codecs()?;
        dbg!(line!());

        let mut registry = Registry::new();
        registry = register_default_interceptors(registry, &mut engine)?;
        dbg!(line!());

        let api = APIBuilder::new()
            .with_media_engine(engine)
            .with_interceptor_registry(registry)
            .build();
        dbg!(line!());

        let config = RTCConfiguration {
            ice_servers: vec![RTCIceServer {
                urls: vec!["stun:stun4.l.google.com:19302".to_owned()],
                ..Default::default()
            }],
            ..Default::default()
        };
        dbg!(line!());

        let peer_connection = Arc::new(api.new_peer_connection(config).await?);
        peer_connection
            .add_transceiver_from_kind(RTPCodecType::Video, &[])
            .await?;
        dbg!(line!());

        let (local_track_chan_tx, mut local_track_chan_rx) =
            tokio::sync::mpsc::channel::<Arc<TrackLocalStaticRTP>>(1);
        dbg!(line!());

        let local_track_chan_tx = Arc::new(local_track_chan_tx);
        dbg!(line!());

        let pc = Arc::downgrade(&peer_connection);
        peer_connection
            .on_track(Box::new(
                move |track: Option<Arc<TrackRemote>>, _receiver: Option<Arc<RTCRtpReceiver>>| {
                    dbg!("HI");
                    if let Some(track) = track {
                        let media_ssrc = track.ssrc();
                        let pc2 = pc.clone();
                        tokio::spawn(async move {
                            let mut result = Result::<usize>::Ok(0);
                            while result.is_ok() {
                                let timeout = tokio::time::sleep(Duration::from_secs(3));
                                dbg!(&media_ssrc);
                                tokio::pin!(timeout);

                                tokio::select! {
                                    _ = timeout.as_mut() =>{
                                        if let Some(pc) = pc2.upgrade(){
                                            result = pc.write_rtcp(&[Box::new(PictureLossIndication{
                                                sender_ssrc: 0,
                                                media_ssrc,
                                            })]).await.map_err(Into::into);
                                        }else{
                                            break;
                                        }
                                    }
                                };
                            }
                        });

                        let local_track_chan_tx2 = Arc::clone(&local_track_chan_tx);
                        tokio::spawn(async move {
                            let local_track = Arc::new(TrackLocalStaticRTP::new(
                                track.codec().await.capability,
                                "video".to_owned(),
                                "webrtc-rs".to_owned(),
                            ));
                            let _ = local_track_chan_tx2.send(Arc::clone(&local_track)).await;

                            while let Ok((rtp, _)) = track.read_rtp().await {
                                if let Err(err) = local_track.write_rtp(&rtp).await {
                                    if Error::ErrClosedPipe != err {
                                        print!(
                                            "output track write_rtp got error: {} and break",
                                            err
                                        );
                                        break;
                                    } else {
                                        print!("output track write_rtp got error: {}", err);
                                    }
                                }
                            }
                        });
                    }

                    Box::pin(async {})
                },
            ))
            .await;
        dbg!(line!());
        peer_connection
            .on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
                println!("Peer Connection State has changed: {}", s);
                Box::pin(async {})
            }))
            .await;
        dbg!(line!());
        peer_connection.set_remote_description(sdp).await?;
        dbg!(line!());

        let answer = peer_connection.create_answer(None).await?;
        dbg!(line!());

        let mut gather_complete = peer_connection.gathering_complete_promise().await;
        dbg!(line!());

        peer_connection.set_local_description(answer).await?;
        dbg!(line!());

        let _ = gather_complete.recv().await;
        dbg!(line!());

        if let Some(sdp) = peer_connection.local_description().await {
            dbg!(line!());
            return Ok(Self {
                channel,
                sdp,
                local_track_chan_rx,
            });
        }
        Err(anyhow::Error::msg("Failed to create Stage"))
    }

    pub async fn subscribe(&mut self, sdp: RTCSessionDescription) -> Result<RTCSessionDescription> {
        if let Some(local_track) = self.local_track_chan_rx.recv().await {
            let mut engine = MediaEngine::default();

            engine.register_default_codecs()?;

            let mut registry = Registry::new();
            registry = register_default_interceptors(registry, &mut engine)?;

            let api = APIBuilder::new()
                .with_media_engine(engine)
                .with_interceptor_registry(registry)
                .build();

            let config = RTCConfiguration {
                ice_servers: vec![RTCIceServer {
                    urls: vec!["stun:stun.l.google.com:19302".to_owned()],
                    ..Default::default()
                }],
                ..Default::default()
            };

            let peer_connection = Arc::new(api.new_peer_connection(config).await?);

            let rtp_sender = peer_connection
                .add_track(Arc::clone(&local_track) as Arc<dyn TrackLocal + Send + Sync>)
                .await?;

            tokio::spawn(async move {
                let mut rtcp_buf = vec![0u8; 1500];
                while let Ok((_, _)) = rtp_sender.read(&mut rtcp_buf).await {}
                Result::<()>::Ok(())
            });

            peer_connection
                .on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
                    println!("Peer Connection State has changed: {}", s);
                    Box::pin(async {})
                }))
                .await;

            peer_connection.set_remote_description(sdp).await?;

            let answer = peer_connection.create_answer(None).await?;

            let mut gather_complete = peer_connection.gathering_complete_promise().await;

            peer_connection.set_local_description(answer).await?;

            let _ = gather_complete.recv().await;

            if let Some(local_desc) = peer_connection.local_description().await {
                Ok(local_desc)
            } else {
                Err(anyhow::Error::msg("Failed to subscribe to Stage"))
            }
        } else {
            Err(anyhow::Error::msg("Failed to subscribe to Stage"))
        }
    }
}
