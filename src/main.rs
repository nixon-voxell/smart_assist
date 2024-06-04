use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_matchbox::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, start_socket)
        .add_systems(Update, receive_messages)
        .add_systems(
            Update,
            send_message.run_if(on_timer(Duration::from_secs(5))),
        )
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
}

fn start_socket(mut commands: Commands) {
    let socket = MatchboxSocket::new_reliable("ws://localhost:3536/main");
    commands.insert_resource(socket);
}

fn send_message(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    let peers: Vec<_> = socket.connected_peers().collect();

    for peer in peers {
        let message = "Hello";
        info!("Sending message: {message:?} to {peer}");
        socket.send(message.as_bytes().into(), peer);
    }
}

fn receive_messages(mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    for (peer, state) in socket.update_peers() {
        info!("{peer}: {state:?}");
    }

    for (id, message) in socket.receive() {
        match std::str::from_utf8(&message) {
            Ok(message) => info!("Received message: {message:?} from {id}"),
            Err(e) => error!("Failed to convert message to string: {e}"),
        }
    }
}
