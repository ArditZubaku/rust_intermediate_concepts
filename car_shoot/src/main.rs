#![warn(clippy::pedantic)]

use rand::{RngExt, seq::IteratorRandom};
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    marble_labels: Vec<String>,
    cars_left: i32,
    spawn_timer: Timer,
}

fn main() {
    let mut game = Game::new();

    let initial_game_state = GameState {
        marble_labels: vec![
            String::from("marble1"),
            String::from("marble2"),
            String::from("marble3"),
        ],
        cars_left: 25,
        spawn_timer: Timer::from_seconds(0.0, TimerMode::Once),
    };

    game.window_settings(Window {
        title: String::from("Car Shoot"),
        ..Default::default()
    });

    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.1);

    let player = game.add_sprite("player", SpritePreset::RacingBarrierRed);
    player.rotation = UP;
    player.scale = 0.5;
    player.translation.y = -325.0;
    player.layer = 10.0;

    let text = game.add_text(
        "cars_left",
        format!("Cars left: {}", initial_game_state.cars_left),
    );
    text.translation = Vec2::new(540.0, -320.0);

    game.add_logic(game_logic);

    game.run(initial_game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    const MARBLE_SPEED: f32 = 600.0;
    const CAR_SPEED: f32 = 250.0;

    let player = engine.sprites.get_mut("player").unwrap();

    if let Some(location) = engine.mouse_state.location() {
        player.translation.x = location.x;
    }

    let player_x = player.translation.x;

    if engine.mouse_state.just_pressed(MouseButton::Left)
        && let Some(label) = game_state.marble_labels.pop()
    {
        let marble = engine.add_sprite(label, SpritePreset::RollingBallBlue);
        marble.translation.x = player_x;
        marble.translation.y = -275.0;
        marble.collision = true;
        engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.4);
    }

    for (label, sprite) in &mut engine.sprites {
        if label.starts_with("marble") {
            sprite.translation.y += MARBLE_SPEED * engine.delta_f32;
        }
    }

    for (label, sprite) in &mut engine.sprites {
        if label.starts_with("car") {
            sprite.translation.x += CAR_SPEED * engine.delta_f32;
        }
    }

    // Clean up sprites which have gone off the screen
    //
    // let mut labels_to_delete = Vec::new();
    // for sprite in engine.sprites.values() {
    //     if sprite.translation.y > 400.0 || sprite.translation.x > 750.0 {
    //         labels_to_delete.push(sprite.label.clone());
    //     }
    // }
    // for label in labels_to_delete {
    //     engine.sprites.remove(&label);
    //     if label.starts_with("marble") {
    //         game_state.marble_labels.push(label);
    //     }
    // }
    engine.sprites.retain(|label, sprite| {
        if sprite.translation.y > 400.0 || sprite.translation.x > 750.0 {
            if label.starts_with("marble") {
                game_state.marble_labels.push(label.clone());
            }
            false // Remove
        } else {
            true // Keep
        }
    });

    // Spawn cars
    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        game_state.spawn_timer =
            Timer::from_seconds(rand::rng().random_range(0.1..1.25), TimerMode::Once);
        if game_state.cars_left > 0 {
            game_state.cars_left -= 1;

            let text = engine.texts.get_mut("cars_left").unwrap();
            text.value = format!("Cars left: {}", game_state.cars_left);

            let label = format!("car{}", game_state.cars_left);
            let car_choices = [
                SpritePreset::RacingCarBlack,
                SpritePreset::RacingCarBlue,
                SpritePreset::RacingCarGreen,
                SpritePreset::RacingCarRed,
                SpritePreset::RacingCarYellow,
            ];
            #[rustfmt::skip]
            let car = engine.add_sprite(
                label,
                *car_choices.iter().choose(&mut rand::rng()).unwrap(),
            );
            car.translation.x = -740.0;
            car.translation.y = rand::rng().random_range(-100.0..325.0);
            car.collision = true;
        }
    }

    // Handle collisions
    for event in engine.collision_events.drain(..) {
        if event.state.is_end() {
            continue;
        }

        if !event.pair.one_starts_with("marble") {
            continue;
        }

        for label in [event.pair.0, event.pair.1] {
            engine.sprites.remove(&label);
            if label.starts_with("marble") {
                game_state.marble_labels.push(label);
            }
        }

        engine.audio_manager.play_sfx(SfxPreset::Confirmation1, 0.1);
    }
}
