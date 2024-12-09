use std::array;
use std::simd::u32x32;

use rayon::prelude::*;

const SEED_BITWIZE: usize = 32;
const SEED_MIN: usize = 0;
const SEED_MAX: usize = 1 << SEED_BITWIZE;
const LUT_LENGTH: usize = (SEED_MAX - SEED_MIN) / SEED_BITWIZE;

pub fn init_lut() -> Box<[u32; LUT_LENGTH]> {
    pub const WORK_TOTAL: usize = 4096; // 必须是2的整数次幂，最好约等于sqrt(核数 * 单线程总时间 / 调度时间)
    pub const WORK_LENGTH: usize = LUT_LENGTH / WORK_TOTAL;
    let mut lut: Box<[u32; LUT_LENGTH]> = Box::new([0; LUT_LENGTH]);

    let offset_u32x32 = u32x32::from_array(array::from_fn(|i| (i * SEED_BITWIZE) as u32));
    // 分块: 多线程
    lut.par_chunks_mut(WORK_LENGTH)
        .enumerate()
        .for_each(|(work_i, work)| {
            // 分块: simd
            work.chunks_exact_mut(SEED_BITWIZE)
                .enumerate()
                .for_each(|(chunk_i, chunk)| {
                    // 分组: offset = 32
                    for offset in 0..SEED_BITWIZE {
                        const M: u32 = 397;
                        const MATRIX_A: u32 = 0x9908b0df;
                        const UPPER_MASK: u32 = 0x80000000;
                        const LOWER_MASK: u32 = 0x7fffffff;

                        // 每1024个种子是1个chunk，内部交错分组（总共32组，每组32个）
                        let seed = u32x32::splat(
                            (work_i * (WORK_LENGTH * SEED_BITWIZE)
                                + chunk_i * (SEED_BITWIZE * SEED_BITWIZE)
                                + offset) as u32,
                        ) + offset_u32x32;

                        // 初始化丐版mt19937的内部状态
                        let mut m = seed;
                        let m0 = m;
                        m = u32x32::splat(1812433253) * (m ^ m >> 30) + u32x32::splat(1);
                        let m1 = m;
                        for i in 2..M + 1 {
                            m = u32x32::splat(1812433253) * (m ^ m >> 30) + u32x32::splat(i);
                        }
                        let mm = m;

                        // 生成一个伪随机数
                        let mut y =
                            (m0 & u32x32::splat(UPPER_MASK)) | (m1 & u32x32::splat(LOWER_MASK));
                        let y_mask = !((y & u32x32::splat(1)) - u32x32::splat(1));
                        y = mm ^ (y >> 1) ^ (y_mask & u32x32::splat(MATRIX_A));

                        y ^= y >> 11;
                        y ^= y << 07 & u32x32::splat(0x9d2c5680);
                        y ^= y << 15 & u32x32::splat(0xefc60000);
                        y ^= y >> 18;

                        // 判断是否是10的整数倍
                        let mut result = y % u32x32::splat(10);
                        result = (result - u32x32::splat(1)) >> (SEED_BITWIZE - 1) as u32;

                        // 移位
                        result <<= u32x32::splat(offset as u32);

                        // 写入lut
                        chunk
                            .iter_mut()
                            .zip(u32x32::to_array(result).iter())
                            .for_each(|(chunk_elem, result_elem)| {
                                *chunk_elem |= result_elem;
                            });
                    }
                });
        });

    lut
}

pub fn is_slime_chunk(lut: &Box<[u32; LUT_LENGTH]>, seed: u32) -> bool {
    lut[seed as usize / SEED_BITWIZE] & (1 << (seed % SEED_BITWIZE as u32)) != 0
}

pub fn get_seed(x: i32, z: i32) -> u32 {
    x as u32 * 0x1f1f1f1f ^ z as u32
}
