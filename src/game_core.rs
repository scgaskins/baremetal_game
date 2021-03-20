#![feature(const_generics)]
#![allow(incomplete_features)]
#![cfg_attr(not(test), no_std)]

#[derive(Copy,Debug,Clone,Eq,PartialEq)]
pub struct SpaceInvadersGame<const WIDTH: usize, const HEIGHT: usize> {
    cells: [[Cell; WIDTH]; HEIGHT]
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
pub struct Position<const WIDTH: usize, const HEIGHT: usize> {
    col: i16, row: i16
}

impl <const WIDTH: usize, const HEIGHT: usize> Position<WIDTH,HEIGHT> {
    pub fn is_legal(&self) -> bool {
        0 <= self.col && self.col < WIDTH as i16 && 0 <= self.row && self.row < HEIGHT as i16
    }

    pub fn row_col(&self) -> (usize, usize) {
        (self.row as usize, self.col as usize)
    }

    pub fn neighbor(&self, d: Dir) -> Position<WIDTH,HEIGHT> {
        match d {
            Dir::N => Position {row: self.row - 1, col: self.col},
            Dir::S => Position {row: self.row + 1, col: self.col},
            Dir::E => Position {row: self.row,     col: self.col + 1},
            Dir::W => Position {row: self.row,     col: self.col - 1}
        }
    }
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Shot<const WIDTH: usize, const HEIGHT: usize> {
    pos: Position<WIDTH,HEIGHT>,
    active: bool
}

impl <const WIDTH: usize, const HEIGHT: usize> Shot<WIDTH,HEIGHT> {
    fn new(pos: Position<WIDTH,HEIGHT>) -> Self {
        Shot {pos, active: false}
    }

    fn fire(&mut self, pos: Position<WIDTH,HEIGHT>) {
        self.pos = pos;
        self.active = true;
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn icon() -> char {
        '|'
    }
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub struct Player<const WIDTH: usize, const HEIGHT: usize> {
    pos: Position<WIDTH,HEIGHT>,
    shot: Shot<WIDTH,HEIGHT>
}

impl <const WIDTH: usize, const HEIGHT: usize> Player<WIDTH,HEIGHT> {
    fn new(pos: Position<WIDTH,HEIGHT>) -> Self {
        Player {pos, shot: Shot::new(pos)}
    }

    fn fire_shot(&mut self) {
        let shot_pos = Position {row: self.pos.row, col: self.pos.col - 1};
        self.shot.fire(shot_pos);
    }
}
