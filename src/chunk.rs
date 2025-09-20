pub struct BlockPosition {
    pub x: i32,
    pub z: i32,
}

impl BlockPosition {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }
}

const fn floor_to_multiple_of_16(n: i32) -> i32 {
    n & !15
}

pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
}

impl From<&BlockPosition> for ChunkPosition {
    fn from(bp: &BlockPosition) -> Self {
        Self {
            x: floor_to_multiple_of_16(bp.x) / 16,
            z: floor_to_multiple_of_16(bp.z) / 16,
        }
    }
}

impl From<&ChunkPosition> for BlockPosition {
    fn from(val: &ChunkPosition) -> Self {
        Self {
            x: val.x * 16,
            z: val.z * 16,
        }
    }
}

impl ChunkPosition {
    pub const fn seed(&self) -> u32 {
        (self.x as u32 * 0x1f1f_1f1f) ^ self.z as u32
    }
}
