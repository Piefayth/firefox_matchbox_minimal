use std::time::Duration;
use crate::common::*;
use bevy::{prelude::*, tasks::IoTaskPool, utils::HashMap};
use matchbox_socket::WebRtcSocket;
use serde::{Deserialize, Serialize};
use urlencoding::encode;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageHeader {
    pub seq: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageBody {
    Heartbeat,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    header: MessageHeader,
    body: MessageBody,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerNegotiation {
    pub seq_out: u64,
    pub last_seen: u64,
}

pub type PeerToNegotiation = HashMap<String, PeerNegotiation>;

const HEARTBEAT_INTERVAL_MS: Duration = Duration::from_millis(1000);

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Connecting).with_system(start_matchbox_socket),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Lobby)
                .with_system(handle_new_peers)
                .with_system(receive)
                .with_system(heartbeat),
        )
        .insert_resource::<PeerToNegotiation>(HashMap::new());
    }
}

pub fn start_matchbox_socket(
    mut commands: Commands,
    task_pool: Res<IoTaskPool>,
    login_state: Res<LoginState>,
    mut game_state: ResMut<State<GameState>>,
) {
    let url = format!("ws://127.0.0.1:3536/{}", encode(&login_state.room));
    let (socket, message_loop) = WebRtcSocket::new(url);

    task_pool.spawn_local(message_loop).detach();
    commands.insert_resource(socket);

    game_state.set(GameState::Lobby).unwrap_or_default();
}

pub fn handle_new_peers(
    mut socket: ResMut<WebRtcSocket>,
    mut peer_to_negotation: ResMut<PeerToNegotiation>,
) {
    let new_peers = socket.accept_new_connections();

    if new_peers.is_empty() {
        return;
    }

    for peer in new_peers {
        peer_to_negotation.insert(peer, PeerNegotiation { ..default() });
    }
}

pub fn heartbeat(
    mut timer: Local<Timer>,
    time: Res<Time>,
    mut socket: ResMut<WebRtcSocket>,
    mut peer_to_negotation: ResMut<PeerToNegotiation>,
) {
    timer.set_duration(HEARTBEAT_INTERVAL_MS);

    timer.tick(time.delta());
    if timer.finished() {
        for (peer, negotiation) in peer_to_negotation.iter_mut() {
            let message = Message {
                header: MessageHeader {
                    seq: negotiation.seq_out,
                },
                body: MessageBody::Heartbeat,
            };

            negotiation.seq_out += 1;

            let serialized = bincode::serialize(&message).unwrap();
            let bytes = serialized.into_boxed_slice();
            socket.send(bytes, peer);
        }

        timer.reset();
    }
}

fn receive(
    mut socket: ResMut<WebRtcSocket>,
    mut peer_to_negotation: ResMut<PeerToNegotiation>,
) {
    for (peer_id, payload) in socket.receive() {
        let negotiation = peer_to_negotation
            .entry(peer_id.clone())
            .or_insert(PeerNegotiation { ..default() });

        let message = bincode::deserialize::<Message>(&payload).unwrap();
        
        negotiation.last_seen = message.header.seq;
    }
}
