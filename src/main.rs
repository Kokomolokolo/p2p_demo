use bevy::{camera::ScalingMode, input::keyboard::Key, prelude::*, transform};
use bevy_matchbox::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // fill the entire brower window
            fit_canvas_to_parent: true,
            // dont steal important keyboard shortcuts
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::srgb(0.55, 0.53, 0.55)))
    .add_systems(Startup, (setup, spawn_player, start_matchbox_socket))
    .add_systems(Update, (move_player, wait_for_players))
    .run();
}


fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection { // OP is a type where player size doest change with distance to camera
            scaling_mode: ScalingMode::AutoMax 
            { 
                max_width: 16.0, // Done so that you cant see more of the world by resizing the window
                max_height: 9.0 
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
struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Sprite {
        color: Color::srgb(0., 0.42, 1.),
        custom_size: Some(Vec2::ONE),
        ..default()
    }));
}

fn move_player(
    mut players: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        direction.y += 1.;
    }
    if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        direction.y -= 1.;
    }
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        direction.x += 1.;
    }
    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        direction.x -= 1.;
    }
    if direction == Vec2::ZERO {
        return;
    }
    
    let move_speed = 7.;
    let move_delta = direction * move_speed * time.delta_secs();

    for mut transform in &mut players {
        transform.translation += move_delta.extend(0.); // Extend, bc translations work in 3d, but the movenent delta has 2d
    }
}

fn wait_for_players(mut socket: ResMut<MatchboxSocket>) {
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

    // todo
}