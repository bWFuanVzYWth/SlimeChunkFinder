use crate::{
    chunk::{self, ChunkPosition},
    slime_chunk::{self, SlimeChunkLut},
};

struct Finder {
    /// 滑动窗口第一个元素最低位对应的区块坐标
    base_position: ChunkPosition,
    /// 滑动窗口，记录一片区域内的史莱姆区块地图\
    /// x跨度就是数组长度，z跨度固定为64
    window: Vec<u64>,
}

impl Finder {
    fn slide(self, lut: &SlimeChunkLut) -> Self {
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
}
