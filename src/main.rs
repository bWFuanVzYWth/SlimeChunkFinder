#![feature(portable_simd)]
#![deny(
    clippy::indexing_slicing,      // 禁止直接索引访问（可能导致越界 panic）
    clippy::unwrap_used,           // 禁止使用 `unwrap`
    // clippy::expect_used,           // 禁止使用 `expect`
    clippy::panic_in_result_fn,    // 禁止在返回 `Result` 的函数中调用 `panic!`
    clippy::todo,                  // 禁止使用 `todo!`
    clippy::unreachable,           // 禁止使用 `unreachable!`
    clippy::unimplemented,          // 禁止使用 `unimplemented!`

    clippy::shadow_unrelated,
    clippy::shadow_same,
    clippy::shadow_unrelated,
)]
#![warn(
    clippy::pedantic,              // 启用所有严格的 Lint
    clippy::cargo,                 // 启用与 Cargo 相关的 Lint
    clippy::nursery                // 启用实验性的 Lint
)]

mod slime_chunk;

fn main() {
    // 初始化：生成/读取lut
    let lut = slime_chunk::init_lut();

    // test
    for z in -10..10 {
        for x in -10..10 {
            print!(
                "{}",
                if slime_chunk::is_slime_chunk(&lut, slime_chunk::get_seed(x, z)) {
                    "# "
                } else {
                    ". "
                }
            );
        }
        println!();
    }
}
