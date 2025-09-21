#![feature(portable_simd)]
#![feature(likely_unlikely)]

// #![warn(
// clippy::unwrap_used,           // 禁止使用 `unwrap`
// clippy::expect_used,           // 禁止使用 `expect`
// clippy::panic_in_result_fn,    // 禁止在返回 `Result` 的函数中调用 `panic!`
// clippy::todo,                  // 禁止使用 `todo!`
// clippy::unreachable,           // 禁止使用 `unreachable!`
// clippy::unimplemented,         // 禁止使用 `unimplemented!`

// clippy::shadow_unrelated,
// clippy::shadow_same,
// )]
// #![warn(
// clippy::pedantic,              // 启用所有严格的 Lint
// clippy::cargo,                 // 启用与 Cargo 相关的 Lint
// clippy::nursery                // 启用实验性的 Lint
// )]

use std::path::Path;
use std::time::Instant;

use crate::{chunk::BlockPosition, finder::Problem};

mod chunk;
mod finder;
mod slime_chunk;

const MASK: [u64; 17] = [
    0b00000111111100000,
    0b00001111111110000,
    0b00011111111111000,
    0b00111111111111100,
    0b01111111111111110,
    0b11111111111111111,
    0b11111111111111111,
    0b11111111111111111,
    0b11111111111111111,
    0b11111111111111111,
    0b11111111111111111,
    0b11111111111111111,
    0b01111111111111110,
    0b00111111111111100,
    0b00011111111111000,
    0b00001111111110000,
    0b00000111111100000,
];

fn main() {
    // 初始化：生成/读取lut
    let init_timer_start = Instant::now();
    let lut_path = "slime_chunk.lut";

    let lut = if Path::new(lut_path).exists() {
        println!("Loading LUT from file...");
        slime_chunk::SlimeChunkLut::load_from_file(lut_path).expect("Failed to load LUT from file")
    } else {
        println!("Generating LUT...");
        let lut = slime_chunk::SlimeChunkLut::new();
        lut.save_to_file(lut_path)
            .expect("Failed to save LUT to file");
        lut
    };

    let init_timer = init_timer_start.elapsed().as_micros();
    println!(
        "Initialization time: {}s",
        (init_timer as f64) / 1_000_000.0
    );

    let from = BlockPosition {
        x: -8_388_608,
        z: -8_388_608,
    };
    let to = BlockPosition {
        x: 8_388_608,
        z: 8_388_608,
    };
    let problem = Problem::new(&from, &to, 60);

    // let from = BlockPosition {
    //     x: -131072,
    //     z: -131072,
    // };
    // let to = BlockPosition {
    //     x: 131072,
    //     z: 131072,
    // };
    // let problem = Problem::new(&from, &to, 50);

    let solve_timer_start = Instant::now();
    problem.solve(&lut, &MASK);
    let solve_timer = solve_timer_start.elapsed().as_micros();
    println!("Solving time: {}s", (solve_timer as f64) / 1_000_000.0);
}
