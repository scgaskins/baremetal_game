#![allow(incomplete_features)]
#![cfg_attr(not(test), no_std)]


use pc_keyboard::{DecodedKey, KeyCode};
use core::borrow::BorrowMut;

const WIDTH: usize = 80;
const HEIGHT: usize = 250;

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct SpaceInvadersGame {
    cells: [[Cell; WIDTH]; HEIGHT],
    status: Status,
    player: Player,
    aliens: Aliens,
    score: u64,
    last_key: Option<KeyCode>
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub enum Status {
    Normal,
    Over
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
#[repr(u8)]
pub enum Cell {
    Empty,
    Barrier
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
#[repr(u8)]
pub enum Dir {
    N, S, E, W
}

impl Dir {
    fn reverse(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E
        }
    }

    fn left(&self) -> Dir {
        match self {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
            Dir::W => Dir::S
        }
    }

    fn right(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N
        }
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Position {
    col: i16, row: i16
}

impl Position {
    pub fn is_legal(&self) -> bool {
        0 <= self.col && self.col < WIDTH as i16 && 0 <= self.row && self.row < HEIGHT as i16
    }

    pub fn row_col(&self) -> (usize, usize) {
        (self.row as usize, self.col as usize)
    }

    pub fn neighbor(&self, d: Dir) -> Position {
        match d {
            Dir::N => Position {row: self.row - 1, col: self.col},
            Dir::S => Position {row: self.row + 1, col: self.col},
            Dir::E => Position {row: self.row,     col: self.col + 1},
            Dir::W => Position {row: self.row,     col: self.col - 1}
        }
    }
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Shot {
    pos: Position,
    active: bool
}

impl Shot {
    fn new(pos: Position) -> Self {Shot {pos, active: false}}

    fn fire(&mut self, pos: Position) {
        self.pos = pos;
        self.active = true;
    }

    fn deactivate(&mut self) { self.active = false;}

    pub fn icon() -> char { '|' }
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Player {
    pos: Position,
    shots: [Shot; 3]
}

impl Player {
    fn new(pos: Position) -> Self {
        let shots = [Shot::new(Position {row: 0, col: 0}); 3];
        Player {pos, shots}
    }

    fn fire_shot(&mut self) {
        let shot_pos = Position {row: self.pos.row, col: self.pos.col - 1};
        for mut shot in self.shots.iter_mut() {
            if !shot.active {
                // fires the first inactive shot
                shot.fire(shot_pos);
                break
            }
        }
    }

    pub fn icon() -> char { '^' }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Alien {
    pos: Position,
    alive: bool
}

impl Alien {
    fn new(pos: Position) -> Self {
        Alien {pos, alive: true}
    }

    fn directly_above_player(&self, player: Player) -> bool {
        player.pos.col == self.pos.col
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Aliens {
    aliens: [[Alien; 24]; 5],
    bottom_row: u32,  // lowest row of aliens
    shot: Shot
}

impl Aliens {
    fn new() -> Self {
        Aliens {aliens: [[Alien::new(Position {row: 0, col: 0}); 24]; 5], bottom_row: 4, shot: Shot::new(Position {row: 0, col: 0})}
    }
}

const START: &'static str =
"#..............................................................................#
#..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@......#
#...@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@.....#
#....@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@....#
#...@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@.....#
#..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@..@......#
#..............................................................................#
#..............................................................................#
#..............................................................................#
#..............................................................................#
#..............................................................................#
#..............................................................................#
#..............................................................................#
#..............................................................................#
#..............................................................................#
#..............................................................................#
#..............................................................................#
###..###..###..###..###..###..###..###..###..###..###..###..###..###..###..##.##
###..###..###..###..###..###..###..###..###..###..###..###..###..###..###..##.##
###..###..###..###..###..###..###..###..###..###..###..###..###..###..###..##.##
#..............................................................................#
#...................................^..........................................#
#..............................................................................#";

impl SpaceInvadersGame {
    pub fn new() -> Self {
        let mut game = SpaceInvadersGame {
          cells: [[Cell::Empty; WIDTH]; HEIGHT],
            status: Status::Normal,
            player: Player::new(Position {row: 0, col: 0}),
            aliens: Aliens::new(),
            score: 0,
            last_key: None
        };
        game.reset();
        game
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn score(&self) -> u64 {
        self.score
    }

    fn reset(&mut self) {
        self.set_up_game();
        self.score = 0;
    }

    // Brings new aliens while keeping score
    fn next_screen(&mut self) {
        self.set_up_game();
    }

    fn set_up_game(&mut self) {
        let mut alien_col = 0;
        let mut alien_row = 0;
        for (row_num, row_chars) in START.split("\n").enumerate() {
            for (col_num, icon) in row_chars.chars().enumerate() {
                self.translate_icon(&mut alien_row, &mut alien_col, row_num, col_num, icon)
            }
        }
        self.status = Status::Normal;
        self.last_key = None;
    }

    fn translate_icon(&mut self, alien_row: &mut usize, alien_col: &mut usize, row: usize, col: usize, icon: char) {
        match icon {
            '.' => self.cells[row][col] = Cell::Empty,
            '#' => self.cells[row][col] = Cell::Barrier,
            '^' => self.player = Player::new(Position {row: row as i16, col: col as i16}),
            '@' => {
                self.aliens.aliens[*alien_row][*alien_col] = Alien::new(Position {row: row as i16, col: col as i16});
                *alien_col += 1;
                if *alien_col >= self.aliens.aliens[0].len() {
                    *alien_col = 0;
                    *alien_row += 1;
                }
            }
            _ => panic!("Invalid char '{}'", icon)
        }
    }

    pub fn cell(&self, p: Position) -> Cell {
        self.cells[p.row as usize][p.col as usize]
    }

    pub fn cell_pos_iter(&self) -> RowColIter {
        RowColIter { row: 0, col: 0 }
    }

    pub fn player_at(&self, p: Position) -> bool {
        p == self.player.pos
    }

    pub fn alien_at(&mut self, p: Position) -> Option<(usize,&mut Alien)> {
        for row in self.aliens.aliens.iter_mut() {
            let outcome = row.iter_mut().enumerate().find(|(col , alien)| alien.pos == p);
            match outcome {
                Some((a, alien)) => {
                    if alien.alive {
                        return Some((a, alien))
                    }
                },
                _ => ()
            }
        }
        return None
    }

    pub fn shot_at(&self, p: Position) -> bool {
        for shot in self.player.shots.iter() {
            if shot.active && shot.pos == p {
                return true
            }
        }
        if self.aliens.shot.active && self.aliens.shot.pos == p {
            return true
        }
        return false
    }

    fn check_collisions(&mut self) {
        for shot in self.player.shots.iter_mut() {
            if shot.active {
                self.check_shot_collision(shot);
            }
        }
        if self.aliens.shot.active {
            self.check_shot_collision(&mut self.aliens.shot);
        }
    }

    fn check_shot_collision(&mut self, shot: &mut Shot) {
        if self.cell(shot.pos) == Cell::Barrier {
            self.cells[shot.pos.row][shot.pos.col] = Cell::Empty;
            shot.active = false;
        } else if self.player_at(shot.pos) {
            self.status = Status::Over;
            shot.active = false;
        } else {
            match self.alien_at(shot.pos) {
                Some((a, alien)) => {
                    alien.alive = false;
                    shot.active = false;
                },
                _ => {}
            }
        }
    }

    pub fn key(&mut self, key: DecodedKey) {
        match self.status {
            Status::Over => {
                match key {
                    DecodedKey::RawKey(KeyCode::S) | DecodedKey::Unicode('s') => self.reset(),
                    _ => {}
                }
            }
            _ => {
                let key = check_valid_key(key);
                if key.is_some() {
                    self.last_key = key;
                }
            }
        }
    }
}

fn check_valid_key(key: DecodedKey) -> Option<KeyCode> {
    match key {
        DecodedKey::RawKey(k) => match k {
            KeyCode::ArrowLeft => Some(k),
            KeyCode::ArrowRight => Some(k),
            KeyCode::Spacebar => Some(k),
            _ => None
        },
        _ => None
    }
}

pub struct RowColIter {
    row: usize, col: usize
}

impl Iterator for RowColIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < HEIGHT {
            let result = Some(Position {row: self.row as i16, col: self.col as i16});
            self.col += 1;
            if self.col == WIDTH {
                self.col = 0;
                self.row += 1;
            }
            result
        } else {
            None
        }
    }
}

