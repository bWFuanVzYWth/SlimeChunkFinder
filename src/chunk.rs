pub struct BlockPosition {
    pub x: i32,
    pub z: i32,
}

const fn floor_to_multiple_of_16(n: i32) -> i32 {
    n & !15
}

#[derive(Clone)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
}

pub struct ChunkRange {
    pub from: ChunkPosition,
    pub dx: i32,
    pub dz: i32,
}

impl ChunkRange {
    pub fn new(from: &ChunkPosition, to: &ChunkPosition) -> Self {
        let x_min = from.x.min(to.x);
        let x_max = from.x.max(to.x);
        let z_min = from.z.min(to.z);
        let z_max = from.z.max(to.z);

        let dx = x_max - x_min + 1;
        let dz = z_max - z_min + 1;

        Self {
            from: ChunkPosition { x: x_min, z: z_min },
            dx,
            dz,
        }
    }
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
