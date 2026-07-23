use core::f32;

use rusty_engine::prelude::*;

#[derive(Debug, Default, Resource)]
struct GameState {
    high_score: u32,
    score: u32,
    ferris_index: i32,
    // spawn_timer: Timer,
}

const PLAYER_LABEL: &str = "player";
const SCORE_LABEL: &str = "score";
const HIGH_SCORE_LABEL: &str = "high_score";

fn main() {
    let mut game = Game::new();

    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.3);

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

    let score = game.add_text(SCORE_LABEL, "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text(HIGH_SCORE_LABEL, "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);

    game.add_logic(game_logic);

    let init_state = GameState::default();
    game.run(init_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // quit if Q is pressed
    if engine.keyboard_state.just_pressed(KeyCode::KeyQ) {
        engine.should_exit = true;
    }

    // keep text near the edges of the screen
    let offset = ((engine.time_since_startup_f64 * 5.0).cos() * 5.0) as f32;
    let score = engine.texts.get_mut(SCORE_LABEL).unwrap();
    score.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
    score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;
    let high_score = engine.texts.get_mut(HIGH_SCORE_LABEL).unwrap();
    high_score.translation.x = -engine.window_dimensions.x / 2.0 + 110.0;
    high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0;

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

            game_state.score += 1;

            let score = engine.texts.get_mut(SCORE_LABEL).unwrap();
            score.value = format!("Score: {}", game_state.score);

            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;
                let high_score = engine.texts.get_mut(HIGH_SCORE_LABEL).unwrap();
                high_score.value = format!("High Score: {}", game_state.high_score);
            }

            engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.3);
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

    // reset score
    if engine.keyboard_state.just_pressed(KeyCode::KeyR) {
        game_state.score = 0;
        let score = engine.texts.get_mut(SCORE_LABEL).unwrap();
        score.value = String::from("Score: 0");
    }
}
