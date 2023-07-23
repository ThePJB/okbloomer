use crate::vector::*;
use crate::vector_i32::*;
use crate::heightmap::*;
use std::collections::HashMap;

pub const CHUNK_XY: usize = 128;
pub const CHUNK_Z: usize = 64;

pub struct OpaqueChunk {
    pub data: [u64; CHUNK_XY * CHUNK_XY],
}

impl OpaqueChunk {
    // true if there is one, false if oob or not one
    pub fn get(&self, pos: Vec3i) -> bool {
        let min = vec3i(0,0,0);
        let max = vec3i(CHUNK_XY as i32 - 1, CHUNK_XY as i32 - 1, CHUNK_Z as i32 - 1);
        if pos < min {return false};
        if pos > max {return false};
        if self.data[pos.x as usize * CHUNK_XY + pos.y as usize] & (1 << pos.z as u64) == 0 {return false};
        return true;
    }
}



// yes there will be f32 vertex data and u16 index data
// do I make this just return a vao or what
// meshmanager should be vao, vbo per chunk

pub struct MeshManager {

}

pub struct OpaqueManager {
    chunks: HashMap<Vec3i, OpaqueChunk>,
}


// well I can't really test correctness until I do meshings and renderings
// but I can print out stats etc.
pub fn generate(coords: Vec3i) -> OpaqueChunk {
    let mut chunk = OpaqueChunk { data: [0; CHUNK_XY * CHUNK_XY] };
    let coords_base = vec3(coords.x as f32 * CHUNK_XY as f32, coords.y as f32 * CHUNK_XY as f32, coords.z as f32);
    for i in 0..CHUNK_XY {
        for j in 0..CHUNK_XY {
            let xy_offset = vec3(i as f32, j as f32, 0.);
            let h = heightmap((coords_base + xy_offset).xy(), 69);
            if h < coords_base.z {
                chunk.data[i * CHUNK_XY + j] = 0;
            } else if h > coords_base.z + CHUNK_Z as f32 {
                chunk.data[i * CHUNK_XY + j] = 0xFFFFFFFFFFFFFFFF;
            } else {
                chunk.data[i * CHUNK_XY + j] = 0xFFFFFFFFFFFFFFFF >> ((h - coords_base.z).round()) as u64;
            }
        }
    }
    chunk
}
