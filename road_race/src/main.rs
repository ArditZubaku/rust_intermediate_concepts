use rusty_engine::prelude::*;

// Define a struct to hold custom data for your game (it can be a lot more complicated than this one!)
#[derive(Resource)]
struct GameState {}

fn main() {
    let mut game = Game::new();

    game.add_logic(game_logic);

    let initial_game_state = GameState {};
    game.run(initial_game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {}
