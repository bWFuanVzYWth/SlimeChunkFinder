use std::fmt::Display;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    chunk::{BlockPosition, ChunkPosition, ChunkRange},
    slime_chunk::SlimeChunkLut,
};

const FINDER_WINDOW_HEIGHT: i32 = 64;

pub struct Finder {
    /// 滑动窗口第一个元素最低位对应的区块坐标
    base_position: ChunkPosition,
    /// 滑动窗口，记录一片区域内的史莱姆区块地图\
    /// x跨度就是数组长度，z跨度固定为64
    window: Vec<u64>,
}

pub struct Solution {
    position: ChunkPosition,
    slime_chunk_count: u32,
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let block_position = BlockPosition::from(&self.position);
        write!(
            f,
            "Solution at ({}, {}) with {} slime chunks",
            block_position.x, block_position.z, self.slime_chunk_count
        )
    }
}

impl Finder {
    pub fn new(start: ChunkPosition, width: usize, lut: &SlimeChunkLut) -> Self {
        let mut tmp = Self {
            base_position: ChunkPosition {
                x: start.x - FINDER_WINDOW_HEIGHT,
                ..start
            },
            window: vec![0; width],
        };
        for _ in 0..FINDER_WINDOW_HEIGHT {
            tmp = tmp.slide(lut);
        }
        tmp
    }

    pub fn slide(self, lut: &SlimeChunkLut) -> Self {
        let base_position = ChunkPosition {
            x: self.base_position.x + 1,
            ..self.base_position
        };
        let window = self
            .window
            .into_iter()
            .enumerate()
            .map(|(z_offset, sub_window)| {
                let position = ChunkPosition {
                    z: base_position.z + z_offset as i32,
                    ..base_position
                };
                (sub_window << 1) | u64::from(lut.is_slime_chunk(position.seed()))
            })
            .collect();
        Self {
            base_position,
            window,
        }
    }

    pub fn find(&self, mask: &[u64], min_count: u32) -> Vec<Solution> {
        self.window
            .windows(mask.len())
            .enumerate()
            .filter_map(|(z_offset, window)| {
                let slime_chunk_count = window
                    .iter()
                    .zip(mask)
                    .map(|(scan_line, scan_mask)| (scan_line & scan_mask).count_ones())
                    .sum();
                if slime_chunk_count > min_count {
                    Some(Solution {
                        position: ChunkPosition {
                            z: self.base_position.z + z_offset as i32,
                            x: self.base_position.x - mask.len() as i32,
                        },
                        slime_chunk_count,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}

pub struct Problem {
    range: ChunkRange,
}

impl Problem {
    pub fn new(
        block_from: &BlockPosition,
        block_to: &BlockPosition,
    ) -> Self {
        let chunk_from = block_from.into();
        let chunk_to = block_to.into();
        let chunk_range = ChunkRange::new(&chunk_from, &chunk_to);
        Self {
            range: chunk_range,
        }
    }

    pub fn break_down(self, mask: &[u64]) -> Vec<Self> {
        const N: usize = 2048;

        // 让finder.window.len()对齐N
        let width = N - mask.len() + 1;
        let n = self.range.dx as usize / width;
        let m = self.range.dx as usize % width;

        let mut sub_problems = (0..n)
            .map(|i| {
                let x = self.range.from.x + (i * width) as i32;
                Self {
                    range: ChunkRange {
                        from: ChunkPosition {
                            x,
                            ..self.range.from
                        },
                        dx: width as i32,
                        ..self.range
                    },
                }
            })
            .collect::<Vec<_>>();

        if m > 0 {
            let x = self.range.from.x + (n * width) as i32;
            sub_problems.push(Self {
                range: ChunkRange {
                    from: ChunkPosition {
                        x,
                        ..self.range.from
                    },
                    dx: m as i32,
                    ..self.range
                },
            })
        }

        sub_problems
    }

    pub fn solve(self, lut: &SlimeChunkLut, mask: &[u64], slime_chunk_count: u32) {
        let sub_problems = self.break_down(mask);
        sub_problems.par_iter().for_each(|sub_problem| {
            let mut finder = Finder::new(
                sub_problem.range.from.clone(),
                sub_problem.range.dx as usize,
                lut,
            );
            for _ in 0..sub_problem.range.dz {
                let solutions = finder.find(mask, slime_chunk_count);
                for solution in solutions {
                    println!("{}", solution);
                }
                finder = finder.slide(lut);
            }
        });
    }
}
