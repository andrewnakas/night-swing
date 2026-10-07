//! What this game is. Its modes, world and time of day are fixed.

use crate::core::modes::Mode;
use crate::core::scene::World;

pub const TITLE: &str = "Night Swing";
pub const TAGLINE: &str = "Web-swing through a moonlit city, steal cars out of the air and lose the police in the dark.";
pub const MODES: &[Mode] = &[Mode::Streets, Mode::Swing];
pub const WORLD: World = World::City;
pub const NIGHT: bool = true;
/// Shown on the controls screen (Tab), after the movement basics.
pub const HELP: &[&str] = &[
    "Q: hold to web-swing, release to fling.  Z: tether two objects.",
    "E: steal a car (do it within 2.5 s of a swing release for an AERIAL JACK).  W/S drive, A/D steer, Space handbrake.",
    "Crime raises your wanted level. Swing over roadblocks and lose the police in the dark.",
];

pub fn mask() -> u8 {
    MODES.iter().fold(0, |m, x| m | x.bit())
}
