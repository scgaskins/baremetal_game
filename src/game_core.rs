#![allow(incomplete_features)]
#![cfg_attr(not(test), no_std)]


use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT};
use core::borrow::BorrowMut;
use pluggable_interrupt_os::println;
// use term::Attr::Standout;

const WIDTH: usize = BUFFER_WIDTH;
const HEIGHT: usize = BUFFER_HEIGHT - 2;
const UPDATE_FREQUENCY: usize = 3;

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct SpaceInvadersGame {
    cells: [[Cell; WIDTH]; HEIGHT],
    status: Status,
    player: Player,
    aliens: Aliens,
    shots: [Shot; 4],
    score: u64,
    last_dir: Option<Dir>,
    countdown: usize,
    fired_shot: bool
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
    active: bool,
    dir: Dir,
    player_fired: bool // true if fired by player, false if by aliens
}

impl Shot {
    fn new(pos: Position) -> Self {Shot {pos, active: false, dir: Dir::N, player_fired: false}}

    fn fire(&mut self, pos: Position, direction: Dir, player: bool) {
        self.pos = pos;
        self.active = true;
        self.dir = direction;
        self.player_fired = player
    }

    fn deactivate(&mut self) { self.active = false;}

    fn fired_by_player(&self) -> bool {self.player_fired}

    fn next_pos(&self) -> Position {self.pos.neighbor(self.dir)}

    pub fn icon() -> char { '|' }
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Player {
    pos: Position,
    active_shots: u8,
    total_shots: u8
}

impl Player {
    fn new(pos: Position) -> Self {
        Player {pos, active_shots: 0, total_shots: 3}
    }

    fn can_fire_shot(&self) -> bool {
        self.active_shots < self.total_shots
    }

    fn get_shot_position(&self) -> Position {
        Position {row: self.pos.row - 1, col: self.pos.col}
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

    pub fn icon() -> char {'@'}
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub struct Aliens {
    aliens: [[Alien; 24]; 5],
    bottom_row: u32,  // lowest row of aliens
    active_shots: u8,
    total_shots: u8,
    dir: Dir,
}

impl Aliens {
    fn new() -> Self {
        Aliens {aliens: [[Alien::new(Position {row: 0, col: 0}); 24]; 5], bottom_row: 4, active_shots: 0, total_shots: 1, dir: Dir::E}
    }

    fn can_fire_shot(&self) -> bool {
        self.active_shots < self.total_shots
    }

    // fn move_aliens(&self) {
    //     for (row_num, row) in self.aliens.aliens.iter().enumerate() {
    //         for (col_num, alien) in row.iter().enumerate() {
    //             if alien.alive && alien.pos == p {
    //
    //                 return Some((row_num, col_num, alien))
    //             }
    //         }
    //     }
    // }
}

const LAST_RAW : &'static i32 = &17;

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
            shots: [Shot::new(Position {row: 0, col: 0}); 4],
            score: 0,
            last_dir: None,
            countdown: 0,
            fired_shot: false
        };
        game.reset();
        game
    }

    pub fn update(&mut self) {
        if self.status == Status::Normal{
            self.move_player();
            self.last_dir = None;
            if self.fired_shot {
                self.fired_shot = false;
                self.player_shoot();
            }
            self.move_aliens();
            self.move_shots();
            self.check_collisions();
        }
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
        self.last_dir = None;
    }

    fn translate_icon(&mut self, alien_row: &mut usize, alien_col: &mut usize, row: usize, col: usize, icon: char) {
        match icon {
            '.' => self.cells[row][col] = Cell::Empty,
            '#' => self.cells[row][col] = Cell::Barrier,
            '^' => self.player = Player::new(Position {row: row as i16, col: col as i16}),
            '@' => {
                self.aliens.aliens[*alien_row][*alien_col] = Alien::new(
                    Position {row: row as i16, col: col as i16}
                );
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

    pub fn alien_at(&self, p: Position) -> Option<(usize,usize,&Alien)> {
        for (row_num, row) in self.aliens.aliens.iter().enumerate() {
            for (col_num, alien) in row.iter().enumerate() {
                if alien.alive && alien.pos == p {
                    return Some((row_num, col_num, alien))
                }
            }
        }
        return None
    }

    pub fn move_aliens(&mut self) {
        let mut will_hit_wall = false;
        for i in 0..self.aliens.aliens.len(){
            for j in 0..self.aliens.aliens.get(0).unwrap().len(){
                let mut alien1 : &mut Alien = &mut self.aliens.aliens[i][j];
                let nextPos  = alien1.pos.neighbor(self.aliens.dir);
                let down_next = alien1.pos.neighbor(Dir::S);
                let(row, col) = nextPos.row_col();
                if !nextPos.is_legal(){
                    will_hit_wall = true;
                }
            }
        }

        if will_hit_wall {
            self.aliens.dir = self.aliens.dir.reverse();
        }

        for i in 0..self.aliens.aliens.len(){
            for j in 0..self.aliens.aliens.get(0).unwrap().len(){
                let mut alien1 : &mut Alien = &mut self.aliens.aliens[i][j];
                let nextPos  = alien1.pos.neighbor(self.aliens.dir);
                let down_next = alien1.pos.neighbor(Dir::S);
                let(row, col) = nextPos.row_col();
                if row as i32 == *LAST_RAW {
                    self.status = Status::Over;
                }else{
                    if will_hit_wall{
                        alien1.pos = down_next ;
                    }else{
                        alien1.pos = nextPos;
                    }
                }
            }
        }

    }

    pub fn shot_at(&self, p: Position) -> bool {
        for shot in self.shots.iter() {
            if shot.active && shot.pos == p {
                return true
            }
        }
        return false
    }

    fn move_player(&mut self) {
        if let Some(dir) = self.last_dir {
            let neighbor = self.player.pos.neighbor(dir);
            if neighbor.is_legal() {
                let (row, col) = neighbor.row_col();
                if self.cells[row][col] != Cell::Barrier {
                    self.player.pos = neighbor;
                }
            }
        }
    }

    fn move_shots(&mut self) {
        for shot in self.shots.iter_mut() {
            if shot.active {
                let new_pos = shot.next_pos();
                if new_pos.is_legal() {
                    shot.pos = new_pos;
                } else {
                    shot.deactivate();
                    if shot.player_fired {
                        self.player.active_shots -= 1;
                    } else {
                        self.aliens.active_shots -= 1;
                    }
                }
            }
        }
    }

    fn check_collisions(&mut self) {
        for i in 0..self.shots.len() {
            if self.shots.get(i).unwrap().active {
                if self.check_shot_collision(i) {
                    self.shots.get_mut(i).unwrap().deactivate();
                    if self.shots.get(i).unwrap().fired_by_player() {
                        self.player.active_shots -= 1;
                    } else {
                        self.aliens.active_shots -= 1;
                    }
                }
            }
        }
    }

    fn check_shot_collision(&mut self, shot_index: usize) -> bool {
        let shot_pos = self.shots.get(shot_index).unwrap().pos;
        if self.cell(shot_pos) == Cell::Barrier {
            let (row, col) = shot_pos.row_col();
            self.cells[row][col] = Cell::Empty;
            return true
        } else if self.player_at(shot_pos) {
            self.status = Status::Over;
            return true
        } else {
            match self.alien_at(shot_pos) {
                Some((row, col, alien)) => {
                    let alien = self.aliens.aliens.get_mut(row).unwrap().get_mut(col).unwrap();
                    alien.alive = false;
                    self.score += 1;
                    return true
                },
                _ => {}
            }
        }
        false
    }

    fn find_shot_for_player(&self) -> Option<usize> {
        if self.player.can_fire_shot() {
            for i in 0..self.shots.len() {
                if !self.shots.get(i).unwrap().active {
                    return Some(i)
                }
            }
        }
        return None
    }

    fn player_shoot(&mut self) {
        let fireable_shot = self.find_shot_for_player();
        match fireable_shot {
            Some(i) => {
                let pos = self.player.get_shot_position();
                let shot: &mut Shot = self.shots.get_mut(i).unwrap();
                shot.fire(pos, Dir::N, true);
                self.player.active_shots += 1;
            },
            None => {}
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
            Status::Normal => {
                match key {
                    DecodedKey::RawKey(KeyCode::F) | DecodedKey::Unicode('f')=> {self.fired_shot = true},
                    DecodedKey::RawKey(k) => match k {
                    KeyCode::ArrowLeft => {
                        self.last_dir = Some(Dir::W);
                    },
                    KeyCode::ArrowRight => {self.last_dir = Some(Dir::E)},
                    _                 => {}
                },
                    _ => {}
                }
            }
        }
    }

    pub fn countdown_complete(&mut self) -> bool {
        if self.countdown == 0 {
            self.countdown = UPDATE_FREQUENCY;
            true
        } else {
            self.countdown -= 1;
            false
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

