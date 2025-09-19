use std::array;
use std::simd::u32x64;

use rayon::prelude::*;

const SEED_BITWISE: usize = 32;
const BLOCK_BITWISE: usize = 2048;

const BLOCK_LENGTH: usize = BLOCK_BITWISE / SEED_BITWISE;
const LUT_LENGTH: usize = (u32::MAX as usize + 1) / SEED_BITWISE;

// 分块：多线程
pub fn init_lut() -> Vec<u32> {
    (0..(LUT_LENGTH / BLOCK_LENGTH) as u32)
        .into_par_iter()
        .map(|chunk_i| {
            let seed_base = chunk_i * (SEED_BITWISE * BLOCK_LENGTH) as u32;
            process_chunk(seed_base)
        })
        .collect::<Vec<_>>()
        .concat()
}

// 分组：simd
fn process_chunk(seed_base: u32) -> [u32; BLOCK_LENGTH] {
    const M: u32 = 397;
    const MATRIX_A: u32 = 0x9908_b0df;
    const UPPER_MASK: u32 = 0x8000_0000;
    const LOWER_MASK: u32 = 0x7fff_ffff;

    let mut result = u32x64::splat(0);

    for offset in 0..(SEED_BITWISE as u32) {
        let seed = u32x64::from_array(array::from_fn(|i| {
            seed_base + offset + (i as u32) * (SEED_BITWISE as u32)
        }));

        // 初始化丐版mt19937的内部状态
        let mut m = seed;
        let m0 = m;
        m = u32x64::splat(1_812_433_253) * (m ^ m >> 30) + u32x64::splat(1);
        let m1 = m;
        for i in 2..=M {
            m = u32x64::splat(1_812_433_253) * (m ^ m >> 30) + u32x64::splat(i);
        }
        let mm = m;

        // 生成一个伪随机数
        let mut y = (m0 & u32x64::splat(UPPER_MASK)) | (m1 & u32x64::splat(LOWER_MASK));
        let y_mask = !((y & u32x64::splat(1)) - u32x64::splat(1));
        y = mm ^ (y >> 1) ^ (y_mask & u32x64::splat(MATRIX_A));

        y ^= y >> 11;
        y ^= y << 7 & u32x64::splat(0x9d2c_5680);
        y ^= y << 15 & u32x64::splat(0xefc6_0000);
        y ^= y >> 18;

        // 判断是否是10的整数倍
        let mut is_slime_chunk = y % u32x64::splat(10);
        is_slime_chunk = (is_slime_chunk - u32x64::splat(1)) >> (SEED_BITWISE - 1) as u32;

        // 压位
        result |= is_slime_chunk << u32x64::splat(offset);
    }

    result.to_array()
}

pub const fn is_slime_chunk(lut: &[u32], seed: u32) -> bool {
    lut[seed as usize / SEED_BITWISE] & (1 << (seed % SEED_BITWISE as u32)) != 0
}

pub const fn get_seed(x: i32, z: i32) -> u32 {
    (x as u32 * 0x1f1f_1f1f) ^ z as u32
}
