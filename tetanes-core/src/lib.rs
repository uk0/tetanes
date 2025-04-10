#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://github.com/lukexor/tetanes/blob/main/assets/linux/icon.png?raw=true",
    html_logo_url = "https://github.com/lukexor/tetanes/blob/main/assets/linux/icon.png?raw=true"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod action;
pub mod apu;
pub mod bus;
pub mod cart;
pub mod debug;
pub mod fs;
pub mod time;
#[macro_use]
pub mod common;
pub mod control_deck;
pub mod cpu;
pub mod error;
pub mod genie;
pub mod input;
pub mod mapper;
pub mod mem;
pub mod ppu;
pub mod sys;
pub mod video;

pub mod prelude {
    //! The prelude re-exports all the common structs/enums used for basic NES emulation.

    pub use crate::{
        action::Action,
        apu::{Apu, Channel},
        cart::Cart,
        common::{Clock, ClockTo, NesRegion, Regional, Reset, ResetKind, Sample},
        control_deck::{Config, ControlDeck, HeadlessMode},
        cpu::Cpu,
        genie::GenieCode,
        input::{FourPlayer, Input, Player},
        mapper::{MappedRead, MappedWrite, Mapper, MapperRevision, Mirrored},
        mem::RamState,
        ppu::{Mirroring, Ppu},
        video::Frame,
    };
}
