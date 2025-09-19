#![feature(portable_simd)]
use std::time::Instant;
mod slime_chunk;

fn main() {
    // 初始化：生成/读取lut
    let init_timer_start = Instant::now();
    let lut = slime_chunk::init_lut();
    let init_timer = init_timer_start.elapsed().as_micros();
    println!("初始化耗时：{}s", (init_timer as f64) / 1000000.0);

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
