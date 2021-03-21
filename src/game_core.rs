#![allow(incomplete_features)]
#![cfg_attr(not(test), no_std)]

static WIDTH: usize = 80;
static HEIGHT: usize = 250;

#[derive(Copy,Debug,Clone,Eq,PartialEq)]
pub struct SpaceInvadersGame {
    cells: [[Cell; WIDTH]; HEIGHT],
    status: Status,
    player: Player,
    aliens: Aliens,
    score: u64,
    last_key: Option<Dir>
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub enum Status {
    Normal,
    Over,
    Empowered
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
#[repr(u8)]
pub enum Cell {
    Empty,
    Barrier,
    PowerDot
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

    fn icon() -> char { '|' }
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Player {
    pos: Position,
    shots: [Shot; 3]
}

impl Player {
    fn new(pos: Position) -> Self {
        shots = [Shot; 3];
        for i in 0..shots.len() {
            shots[i] = Shot::new();
        }
        Player {pos, shots}
    }

    fn fire_shot(&mut self) {
        let shot_pos = Position {row: self.pos.row, col: self.pos.col - 1};
        for shot in self.shots.iter() {
            if !shot.active {
                // fires the first inactive shot
                shot.fire(shot_pos);
                break
            }
        }
    }

    fn icon() -> char { '^' }
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

pub struct Aliens {
    aliens: [[Alien; 24]; 5],
    bottom_row: u32,  // lowest row of aliens
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
