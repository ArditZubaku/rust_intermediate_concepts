use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {}

fn main() {
    let mut game = Game::new();

    // setup game here

    let init_state = GameState {};
    game.run(init_state);
}
