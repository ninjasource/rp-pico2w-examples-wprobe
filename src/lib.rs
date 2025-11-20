#![no_std]

use embassy_rp::block::ImageDef;
pub mod network;
pub mod radio;

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();
