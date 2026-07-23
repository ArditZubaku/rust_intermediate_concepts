use core::f32;

use rusty_engine::prelude::*;

#[derive(Debug, Default, Resource)]
struct GameState {
    // high_score: u32,
    current_score: u32,
    ferris_index: i32,
    // spawn_timer: Timer,
}

const PLAYER_LABEL: &str = "player";

// impl Default for GameState {
//     fn default() -> Self {
//         Self {
//             // high_score: 0,
//             current_score: 0,
//             ferris_index: 0,
//             // spawn_timer: Timer::from_seconds(1.0, TimerMode::Once),
//         }
//     }
// }

fn main() {
    let mut game = Game::new();

    let player = game.add_sprite(PLAYER_LABEL, SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    // player.rotation = std::f32::consts::FRAC_PI_2;
    // player.rotation = UP;
    player.rotation = SOUTH_WEST;
    player.scale = 1.5;
    player.layer = 1.0; // when on the same layer it's non-deterministic
    player.collision = true;

    // let temporary = game.add_sprite("temporary", SpritePreset::RacingCarRed);
    // temporary.translation = Vec2::new(30.0, 0.0);
    // temporary.layer = 999.0; // the highest layer possible to be set

    game.add_logic(game_logic);

    let init_state = GameState::default();
    game.run(init_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // handle collision
    for event in engine.collision_events.drain(..) {
        // println!("{:?}", event);
        if event.state == CollisionState::Begin && event.pair.one_starts_with(PLAYER_LABEL) {
            // remove the sprit the player collided with
            for label in [event.pair.0, event.pair.1] {
                if label != PLAYER_LABEL {
                    engine.sprites.remove(&label);
                }
            }

            game_state.current_score += 1;
            println!("Game state {:?}", game_state);
        }
    }

    // handle movement
    let player = engine.sprites.get_mut(PLAYER_LABEL).unwrap();

    const MOVEMENT_SPEED: f32 = 100.0;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowUp, KeyCode::KeyW])
    {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowDown, KeyCode::KeyS])
    {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowRight, KeyCode::KeyD])
    {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::ArrowLeft, KeyCode::KeyA])
    {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }

    // handle mouse input
    if engine.mouse_state.just_pressed(MouseButton::Left)
        && let Some(mouse_location) = engine.mouse_state.location()
    {
        let label = format!("ferris{}", game_state.ferris_index);
        game_state.ferris_index += 1;
        let ferris = engine.add_sprite(label.clone(), SpritePreset::RacingCarYellow);
        ferris.translation = mouse_location;
        ferris.collision = true;
    }
}
