use std::collections::HashMap;
use crate::vector::*;

const CHUNK_XY: usize = 128;
const CHUNK_Z: usize = 64;

pub struct Game {
    opaque: OpaqueManager,

}

pub struct Vec3i {
    x: i32,
    y: i32,
    z: i32,
}

pub struct OpaqueChunk {
    data: [u64; CHUNK_XY * CHUNK_XY],
}

// yes there will be f32 vertex data and u16 index data
// do I make this just return a vao or what
// meshmanager should be vao, vbo per chunk

pub struct MeshManager {

}

pub struct OpaqueManager {
    chunks: HashMap<Vec3i, OpaqueChunk>,
}

pub fn heightmap(p: V2) -> f32 {

}

pub fn generate(coords: Vec3i) -> OpaqueChunk {
    let mut chunk = OpaqueChunk { data: [0; CHUNK_XY * CHUNK_XY]};
    let coords_base = vec3(coords.x as f32 * CHUNK_XY as f32, coords.y as f32 * CHUNK_XY as f32, coords.z);
    for i in 0..CHUNK_XY {
        for j in 0..CHUNK_XY {
            let xy_offset = vec3(i as f32, j as f32, 0.);
            let h = heightmap((coords_base + xy_offset).xy());
            if h < coords_base.z {
                chunk.data[i * CHUNK_XY + j] = 0;
            } else if h > coords_base.z + CHUNK_Z {
                chunk.data[i * CHUNK_XY + j] = 0xFFFFFFFFFFFFFFFF;
            } else {
                chunk.data[i * CHUNK_XY + j] = 0xFFFFFFFFFFFFFFFF >> ((h - coords_base.z).round()) as u64;
            }
        }
    }
    chunk
}

pub struct RawMesh {
    vertex: Vec<u8>,
    index: Vec<u8>,
}

//  mesh on cpu for now, eye for making this a geometry shader
pub fn mesh(chunk: OpaqueChunk) -> RawMesh {

}

// upload: RawMesh -> vao,vbo

// could defs have "All one or the other" optimization
// does introduce a branch and non homogenous / different class
//  but it is a majority of instances too.
// maybe just goes in chunk_properties: small struct. AllOpaque: bool, AllTransparent: bool
// be good if hashing shit was the same for all
// see it could have more elaborate structures in it. could have a much bigger one that was explicit of structures etc.
// that was sparse instead. some sparse, seems powerful
// and dont keep chunks for shit thats non default. like tree growth if theres no trees in the chunk
// ideally reusing indexes with a fixed offset