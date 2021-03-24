#![no_std]
#![no_main]

use lazy_static::lazy_static;
use spin::Mutex;
use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::HandlerTable;
use pluggable_interrupt_os::vga_buffer::clear_screen;
use crossbeam::atomic::AtomicCell;
use pluggable_interrupt_os::println;
use baremetal_game::game_core::SpaceInvadersGame;

pub mod game_core;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .start()
}

lazy_static! {
    static ref GAME: Mutex<SpaceInvadersGame> = Mutex::new(SpaceInvadersGame::new());
}

fn tick() {
    baremetal_game::tick(&mut GAME.lock())
}

fn key(key: DecodedKey) {
    GAME.lock().key(key);
}

/*fn startup() {
    clear_screen();
}
 */