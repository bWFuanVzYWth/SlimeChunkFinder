use std::fmt::Display;

use crate::{
    chunk::{self, BlockPosition, ChunkPosition},
    slime_chunk::{self, SlimeChunkLut},
};

const FINDER_WINDOW_HEIGTH: i32 = 64;

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
                x: start.x - FINDER_WINDOW_HEIGTH,
                ..start
            },
            window: vec![0; width],
        };
        for _ in 0..FINDER_WINDOW_HEIGTH {
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
                            ..self.base_position
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
