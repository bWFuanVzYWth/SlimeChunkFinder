#![feature(portable_simd)]

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
