use std::{fmt::Display, hint::unlikely};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    chunk::{BlockPosition, ChunkPosition, ChunkRange},
    slime_chunk::SlimeChunkLut,
};

const FINDER_WINDOW_Z: i32 = 64;

pub struct Finder {
    /// The chunk coordinates corresponding to the lowest bit of the first element of the sliding window.
    base_position: ChunkPosition,
    /// Sliding window, recording a map of slime chunks in an area.\
    /// x span is the array length, z span is fixed at 64
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
    pub fn new(start: &ChunkPosition, width: usize, lut: &SlimeChunkLut) -> Self {
        let mut tmp = Self {
            base_position: ChunkPosition {
                x: start.x - FINDER_WINDOW_Z,
                ..start.clone()
            },
            window: vec![0; width],
        };
        for _ in 0..FINDER_WINDOW_Z {
            tmp.slide(lut);
        }
        tmp
    }

    pub fn slide(&mut self, lut: &SlimeChunkLut) {
        self.base_position.x += 1;
        for (z_offset, sub_window) in self.window.iter_mut().enumerate() {
            let position = ChunkPosition {
                z: self.base_position.z + z_offset as i32,
                x: self.base_position.x + FINDER_WINDOW_Z - 1,
            };
            *sub_window = (*sub_window << 1) | u64::from(lut.is_slime_chunk(position.seed()));
        }
    }

    pub fn find(&self, mask: &[u64], min_count: u32) {
        self.window
            .windows(mask.len())
            .enumerate()
            .for_each(|(z_offset, window)| {
                let slime_chunk_count = window
                    .iter()
                    .zip(mask)
                    .map(|(scan_line, scan_mask)| (scan_line & scan_mask).count_ones())
                    .sum();
                if unlikely(slime_chunk_count > min_count) {
                    let solution = Solution {
                        position: ChunkPosition {
                            z: self.base_position.z + z_offset as i32,
                            x: self.base_position.x,
                        },
                        slime_chunk_count,
                    };
                    println!("{solution}");
                }
            })
    }
}

#[derive(PartialEq, Debug)]
pub struct Problem {
    slime_chunk_count: u32,
    range: ChunkRange,
}

impl Problem {
    pub fn new(
        block_from: &BlockPosition,
        block_to: &BlockPosition,
        slime_chunk_count: u32,
    ) -> Self {
        let chunk_from = block_from.into();
        let chunk_to = block_to.into();
        let chunk_range = ChunkRange::new(&chunk_from, &chunk_to);
        Self {
            range: chunk_range,
            slime_chunk_count,
        }
    }

    pub fn break_down(self, mask: &[u64]) -> Vec<Self> {
        const N: usize = 2048;

        // 让finder.window.len()对齐N
        let width = N - mask.len() + 1;
        let n = self.range.dz as usize / width;
        let m = self.range.dz as usize % width;

        let mut sub_problems = (0..n)
            .map(|i| {
                let z = self.range.from.z + (i * width) as i32;
                Self {
                    range: ChunkRange {
                        from: ChunkPosition {
                            z,
                            ..self.range.from
                        },
                        dz: width as i32,
                        ..self.range
                    },
                    ..self
                }
            })
            .collect::<Vec<_>>();

        if m > 0 {
            let z = self.range.from.z + (n * width) as i32;
            sub_problems.push(Self {
                range: ChunkRange {
                    from: ChunkPosition {
                        z,
                        ..self.range.from
                    },
                    dz: m as i32,
                    ..self.range
                },
                ..self
            });
        }

        sub_problems
    }

    pub fn solve(self, lut: &SlimeChunkLut, mask: &[u64]) {
        let sub_problems = self.break_down(mask);
        sub_problems.par_iter().for_each(|sub_problem| {
            let mut finder =
                Finder::new(&sub_problem.range.from, sub_problem.range.dz as usize, lut);
            for _ in 0..sub_problem.range.dx {
                finder.find(mask, sub_problem.slime_chunk_count);
                finder.slide(lut);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_break_down() {
        let problem = Problem {
            range: ChunkRange {
                from: ChunkPosition { x: 0, z: 0 },
                dx: 2048,
                dz: 4096,
            },
            slime_chunk_count: 45,
        };

        let mask = vec![0xFF; 32];
        let sub_problems = problem.break_down(&mask);

        assert_eq!(sub_problems.len(), 3);

        assert_eq!(
            sub_problems[0],
            Problem {
                range: ChunkRange {
                    from: ChunkPosition { x: 0, z: 0 },
                    dx: 2048,
                    dz: 2017,
                },
                slime_chunk_count: 45
            }
        );

        assert_eq!(
            sub_problems[1],
            Problem {
                range: ChunkRange {
                    from: ChunkPosition { x: 0, z: 2017 },
                    dx: 2048,
                    dz: 2017,
                },
                slime_chunk_count: 45
            }
        );

        assert_eq!(
            sub_problems[2],
            Problem {
                range: ChunkRange {
                    from: ChunkPosition { x: 0, z: 4034 },
                    dx: 2048,
                    dz: 62,
                },
                slime_chunk_count: 45
            }
        );
    }
}
