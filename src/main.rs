#![feature(portable_simd)]
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

use std::time::Instant;

use crate::{
    chunk::{BlockPosition, ChunkPosition},
    finder::Finder,
};

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
    let lut = slime_chunk::SlimeChunkLut::new();
    let init_timer = init_timer_start.elapsed().as_micros();
    println!("初始化耗时：{}s", (init_timer as f64) / 1000000.0);

    // test
    // for z in -10..10 {
    //     for x in -10..10 {
    //         let chunk_position = ChunkPosition { x, z };
    //         print!(
    //             "{}",
    //             if lut.is_slime_chunk(chunk_position.seed()) {
    //                 "# "
    //             } else {
    //                 ". "
    //             }
    //         );
    //     }
    //     println!();
    // }

    let start = BlockPosition {
        x: -8388608,
        z: -8388608,
    };
    let mut finder = Finder::new(ChunkPosition::from(&start), 1048576, &lut);
    for _ in 0..1048576 {
        let solutions = finder.find(&MASK, 55);
        for solution in solutions {
            println!("{}", solution);
        }
        finder = finder.slide(&lut);
    }
}
