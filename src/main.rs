mod board;
mod game;
mod piece;
use macroquad::prelude::*;

use crate::{board::render_board, game::Game};

fn window_conf() -> Conf {
    Conf {
        window_title: "tet_rs".to_owned(),
        window_width: 250,
        window_height: 1000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    loop {
        clear_background(RED);

        if is_key_pressed(KeyCode::Left) {
            game.move_player_left()
        }
        if is_key_pressed(KeyCode::Right) {
            game.move_player_right()
        }
        if is_key_pressed(KeyCode::Up) {
            game.move_player_up();
        }
        if is_key_pressed(KeyCode::Down) {
            game.move_player_down();
        }
        if is_key_pressed(KeyCode::Space) {
            game.rotate_player();
        }

        game.render();

        next_frame().await
    }
}
