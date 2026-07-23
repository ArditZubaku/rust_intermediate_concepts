use rusty_engine::prelude::*;

#[derive(Debug, Resource)]
struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

fn main() {
    let mut game = Game::new();

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    // player.rotation = std::f32::consts::FRAC_PI_2;
    // player.rotation = UP;
    player.rotation = SOUTH_WEST;
    player.scale = 1.5;
    player.layer = 1.0; // when on the same layer it's non-deterministic

    // let temporary = game.add_sprite("temporary", SpritePreset::RacingCarRed);
    // temporary.translation = Vec2::new(30.0, 0.0);
    // temporary.layer = 999.0; // the highest layer possible to be set

    game.add_logic(game_logic);

    let init_state = GameState::default();
    game.run(init_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // println!("Game state {:?}", game_state);
}
