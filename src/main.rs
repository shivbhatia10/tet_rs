mod board;
mod game;
mod piece;
use macroquad::prelude::*;

use crate::game::Game;

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
    let mut last_drop = get_time();
    loop {
        clear_background(RED);

        if is_key_pressed(KeyCode::Left) {
            game.move_player_left()
        }
        if is_key_pressed(KeyCode::Right) {
            game.move_player_right()
        }
        if is_key_pressed(KeyCode::Down) {
            game.move_player_down();
        }
        if is_key_pressed(KeyCode::Space) {
            game.rotate_player();
        }

        let now = get_time();
        if now - last_drop >= 1.0 {
            game.move_player_down();
            last_drop = now;
        }

        game.check_if_piece_has_terminated();
        game.render();

        next_frame().await
    }
}
