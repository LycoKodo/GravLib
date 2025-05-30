#![no_main]
#![no_std]

mod GravLib;

use vexide::prelude::*;
use crate::GravLib::actuator;
use crate::GravLib::pid;

struct Robot {}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        println!("Autonomous!");
    }

    async fn driver(&mut self) {
        println!("Driver!");
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let robot = Robot {};

    robot.compete().await;
}