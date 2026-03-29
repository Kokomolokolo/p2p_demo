use bevy::{
    camera::ScalingMode, input::keyboard::Key, platform::collections::HashMap, prelude::*,
    transform,
};
use bevy_ggrs::{AddRollbackCommandExtension, GgrsConfig};
use bevy_ggrs::{
    GgrsPlugin, GgrsSchedule, LocalInputs, LocalPlayers, PlayerInputs, ReadInputs, RollbackApp,
    ggrs,
};
use bevy_matchbox::prelude::*;

use input::*;
use componets::*;

mod input;
mod componets;

// Config-Typ definieren
// u8, input type: 4 directions + fire fits in a single byte
// PeerIs: address type of peers
type Config = GgrsConfig<u8, PeerId>;



fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // fill the entire brower window
                    fit_canvas_to_parent: true,
                    // dont steal important keyboard shortcuts
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            GgrsPlugin::<Config>::default(),
        ))
        .rollback_component_with_clone::<Transform>()
        .insert_resource(ClearColor(Color::srgb(0.55, 0.53, 0.55)))
        .add_systems(Startup, (setup, start_matchbox_socket))
        .add_systems(Update, wait_for_players)
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(GgrsSchedule, move_players)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            // OP is a type where player size doest change with distance to camera
            scaling_mode: ScalingMode::AutoMax {
                max_width: 16.0, // Done so that you cant see more of the world by resizing the window
                max_height: 9.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn start_matchbox_socket(mut commands: Commands) {
    // Part before ? is an id, if we want to host multiple games on one matchbox server
    // after ? is for 2 players if i understood right
    let room_url = "wss://www.okoloki.com/matchbox/p2p_demo?next=2";
    //let room_url = "wss://okoloki.com/matchbox/p2pdems?next=2";

    info!("connectiog to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}


fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            Player { handle: 0 },
            Transform::from_translation(Vec3::new(-2., 0.0, 0.0)),
            Sprite {
                color: Color::srgb(0., 0.42, 1.),
                custom_size: Some(Vec2::ONE),
                ..default()
            },
        
        ))
        .add_rollback();

    commands
        .spawn((
            Player { handle: 1 },
            Transform::from_translation(Vec3::new(2., 0.0, 0.0)),
            Sprite {
                color: Color::srgb(0., 0.42, 0.),
                custom_size: Some(Vec2::ONE),
                ..default()
            },
        
        ))
        .add_rollback();
}

fn wait_for_players(mut commands: Commands, mut socket: ResMut<MatchboxSocket>) {
    if socket.get_channel(0).is_err() {
        return; // already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // waiting for more players
    }

    info!("All peers have joined, going in game");

    // crate a ggrs P2P session
    let mut session_builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    // Add players
    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to conect player!")
    }

    // move channels out of soccet, bc ggrs takes ownership of it
    let channel = socket.take_channel(0).unwrap();

    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));
    
    spawn_player(commands);
}



fn move_players(
    mut players: Query<(&mut Transform, &Player)>,
    inputs: Res<PlayerInputs<Config>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut players {
        
        let (input, _) = inputs[player.handle];
        
        let direction = direction(input);
        
        if direction == Vec2::ZERO {
            continue;
        }
    
        let move_speed = 5.;
        let move_delta = (direction * move_speed * time.delta_secs()).extend(0.0); // Extend, bc translations work in 3d, but the movenent delta has 2d
    
        transform.translation += move_delta; 

    }
}

