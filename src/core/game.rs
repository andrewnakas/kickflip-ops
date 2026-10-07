//! What this game is. Its modes, world and time of day are fixed.

use crate::core::modes::Mode;
use crate::core::scene::World;

pub const TITLE: &str = "Kickflip Ops";
pub const TAGLINE: &str = "A military shooter where you can drop onto a skateboard any time, in a city with a block quarry you can dig and build.";
pub const MODES: &[Mode] = &[Mode::Skate, Mode::Warfare, Mode::Blocks];
pub const WORLD: World = World::City;
pub const NIGHT: bool = false;
/// Shown on the controls screen (Tab), after the movement basics.
pub const HELP: &[&str] = &[
    "G: hop on or off the board.  On the board: W push, Space ollie (hold to charge), J/K/L/U/H flips, Y grab.",
    "LMB fire, RMB aim, R reload, 1-3 weapons, 5/6/7 killstreaks.  4: blocks (LMB break, RMB place, T TNT).",
    "A kill within 1.5 s of a trick scores <TRICK> KILL. Explosions carve the block quarry; rail blocks are grindable.",
];

pub fn mask() -> u8 {
    MODES.iter().fold(0, |m, x| m | x.bit())
}
