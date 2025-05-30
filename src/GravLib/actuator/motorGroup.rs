extern crate alloc;

use alloc::vec::Vec;

use vexide::prelude::Motor;

pub struct MotorGroup {
    ports: Vec<i32>,
    cartridge: vexide::devices::smart::Motor,
}