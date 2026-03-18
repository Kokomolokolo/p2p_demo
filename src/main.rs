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

// Config-Typ definieren
// u8, input type: 4 directions + fire fits in a single byte
// PeerIs: address type of peers
type Config = GgrsConfig<u8, PeerId>;

// Bit mask, for the diffrent inputs
const INPUT_UP: u8 = 1 << 0; // 0000 0001
const INPUT_DOWN: u8 = 1 << 1; // 0000 0010
const INPUT_LEFT: u8 = 1 << 2; // etc.
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_FIRE: u8 = 1 << 4;

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
    let room_url = "ws://127.0.0.1:3536/p2p_demo?next=2";

    info!("connectiog to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}

#[derive(Component)]
struct Player {
    handle: usize,
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

fn read_local_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        let mut input: u8 = 0;

        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            input |= INPUT_UP;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            input |= INPUT_DOWN;
        }
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            input |= INPUT_LEFT
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            input |= INPUT_RIGHT;
        }
        if keys.any_pressed([KeyCode::Space, KeyCode::Enter]) {
            input |= INPUT_FIRE;
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<Config>(local_inputs));
}

fn move_players(
    mut players: Query<(&mut Transform, &Player)>,
    inputs: Res<PlayerInputs<Config>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut players {
        
        let (input, _) = inputs[player.handle];
        
        let mut direction = Vec2::ZERO;
        
        if input & INPUT_UP != 0 {
            direction.y += 1.;
        }
        if input & INPUT_DOWN != 0 {
            direction.y -= 1.;
        }
        if input & INPUT_RIGHT != 0 {
            direction.x += 1.;
        }
        if input & INPUT_LEFT != 0 {
            direction.x -= 1.;
        }
        if direction == Vec2::ZERO {
            continue;
        }
    
        let move_speed = 5.;
        let move_delta = (direction * move_speed * time.delta_secs()).extend(0.0); // Extend, bc translations work in 3d, but the movenent delta has 2d
    
        transform.translation += move_delta; 

    }
}
