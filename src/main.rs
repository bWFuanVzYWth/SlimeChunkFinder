#![feature(portable_simd)]

#![warn(
clippy::unwrap_used,           // 禁止使用 `unwrap`
clippy::expect_used,           // 禁止使用 `expect`
clippy::panic_in_result_fn,    // 禁止在返回 `Result` 的函数中调用 `panic!`
clippy::todo,                  // 禁止使用 `todo!`
clippy::unreachable,           // 禁止使用 `unreachable!`
clippy::unimplemented,          // 禁止使用 `unimplemented!`

clippy::shadow_unrelated,
clippy::shadow_same,
)]
#![warn(
clippy::pedantic,              // 启用所有严格的 Lint
clippy::cargo,                 // 启用与 Cargo 相关的 Lint
clippy::nursery                // 启用实验性的 Lint
)]

use std::time::Instant;

use crate::chunk::ChunkPosition;

mod chunk;
mod finder;
mod slime_chunk;

fn main() {
    // 初始化：生成/读取lut
    let init_timer_start = Instant::now();
    let lut = slime_chunk::SlimeChunkLut::new();
    let init_timer = init_timer_start.elapsed().as_micros();
    println!("初始化耗时：{}s", (init_timer as f64) / 1000000.0);

    // test
    for z in -10..10 {
        for x in -10..10 {
            let chunk_position = ChunkPosition { x, z };
            print!(
                "{}",
                if lut.is_slime_chunk(chunk_position.seed()) {
                    "# "
                } else {
                    ". "
                }
            );
        }
        println!();
    }
}
