use std::array;
use std::simd::u32x32;

use rayon::prelude::*;

const SEED_BITWISE: usize = 32;
const SEED_MIN: usize = 0;
const SEED_MAX: usize = 1 << SEED_BITWISE;
const LUT_LENGTH: usize = (SEED_MAX - SEED_MIN) / SEED_BITWISE;

// 分块：多线程
pub fn init_lut() -> Vec<u32> {
    (0..(LUT_LENGTH / SEED_BITWISE))
        .into_par_iter()
        .map(|chunk_i| {
            let seed_base = chunk_i * (SEED_BITWISE * SEED_BITWISE);
            process_chunk(seed_base)
        })
        .collect::<Vec<_>>()
        .concat()
}

// 分组：simd
fn process_chunk(seed_base: usize) -> [u32; 32] {
    const M: u32 = 397;
    const MATRIX_A: u32 = 0x9908_b0df;
    const UPPER_MASK: u32 = 0x8000_0000;
    const LOWER_MASK: u32 = 0x7fff_ffff;

    let mut result = u32x32::splat(0);

    for offset in 0..SEED_BITWISE {
        // 每1024个种子是1个chunk，内部交错分组（总共32组，每组32个）
        let seed = u32x32::from_array(array::from_fn(|i| {
            (seed_base + offset + i * SEED_BITWISE) as u32
        }));

        // 初始化丐版mt19937的内部状态
        let mut m = seed;
        let m0 = m;
        m = u32x32::splat(1812433253) * (m ^ m >> 30) + u32x32::splat(1);
        let m1 = m;
        for i in 2..=M {
            m = u32x32::splat(1812433253) * (m ^ m >> 30) + u32x32::splat(i);
        }
        let mm = m;

        // 生成一个伪随机数
        let mut y = (m0 & u32x32::splat(UPPER_MASK)) | (m1 & u32x32::splat(LOWER_MASK));
        let y_mask = !((y & u32x32::splat(1)) - u32x32::splat(1));
        y = mm ^ (y >> 1) ^ (y_mask & u32x32::splat(MATRIX_A));

        y ^= y >> 11;
        y ^= y << 7 & u32x32::splat(0x9d2c5680);
        y ^= y << 15 & u32x32::splat(0xefc60000);
        y ^= y >> 18;

        // 判断是否是10的整数倍
        let mut is_slime_chunk = y % u32x32::splat(10);
        is_slime_chunk = (is_slime_chunk - u32x32::splat(1)) >> (SEED_BITWISE - 1) as u32;

        // 压位
        result |= is_slime_chunk << u32x32::splat(offset as u32);
    }

    result.to_array()
}

pub const fn is_slime_chunk(lut: &[u32], seed: u32) -> bool {
    lut[seed as usize / SEED_BITWISE] & (1 << (seed % SEED_BITWISE as u32)) != 0
}

pub const fn get_seed(x: i32, z: i32) -> u32 {
    (x as u32 * 0x1f1f1f1f) ^ z as u32
}
