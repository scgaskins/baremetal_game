#![cfg_attr(not(test), no_std)]


use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, plot_str, plot_num, clear_row, ColorCode, Color};
pub mod game_core;

use crate::game_core::{SpaceInvadersGame, Status, Cell, Position, Alien, Player, Shot};

const GAME_HEIGHT: usize = BUFFER_HEIGHT - 2;
const HEADER_SPACE: usize = BUFFER_HEIGHT - GAME_HEIGHT;


pub type MainGame = SpaceInvadersGame;

pub fn tick(game: &mut MainGame) {
    if game.countdown_complete() {
        game.update();
        draw(game);
    }
}

fn draw(game: &MainGame) {
    draw_header(game);
    draw_board(game);
}

fn draw_header(game: &MainGame) {
    match game.status() {
        Status::Normal => draw_normal_header(game),
        Status::Over => draw_game_over_header(game)
    }
}

fn draw_normal_header(game: &MainGame) {
    clear_row(1, Color::Black);
    let header_color = ColorCode::new(Color::White, Color::Black);
    let score_text = "Score:";
    clear_row(0, Color::Black);
    clear_row(1, Color::Black);
    plot_str(score_text, 0, 0, header_color);
    plot_num(game.score() as isize, score_text.len() + 1, 0, header_color);
}

fn draw_subheader(subheader: &str) {
    plot_str(subheader, 0, 1, ColorCode::new(Color::LightRed, Color::Black));
}

fn draw_game_over_header(game: &MainGame) {
    draw_normal_header(game);
    draw_subheader("Game over. Press S to restart.");
}

fn draw_board(game: &MainGame) {
    for p in game.cell_pos_iter() {
        let (row, col) = p.row_col();
        let (c, color) = get_icon_color(game, p, &game.cell(p));
        plot(c, col, row + HEADER_SPACE, color);
    }
}

fn get_icon_color(game: &MainGame, p: Position, cell: &Cell) -> (char, ColorCode) {
    let (icon, foreground) =
        if game.player_at(p) {
            (match game.status() {
                Status::Over => '*',
                _ => Player::icon()
            }, Color::Yellow)
        } else {
            if game.alien_at(p).is_some() {

                ('@', Color::Green)
            } else if game.shot_at(p) {
                (Shot::icon(), Color::Red)
            } else {
                match cell {
                    Cell::Empty => ('.', Color::White),
                    Cell::Barrier => ('#', Color::Blue)
                }
            }
        };
    (icon, ColorCode::new(foreground, Color::Black))
}
