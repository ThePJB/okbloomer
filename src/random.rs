use std::f32::consts::PI;

pub fn hash(mut state: u32) -> u32 {
    state = (state ^ 2747636419).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state = (state ^ (state >> 16)).wrapping_mul(2654435769);
    state
}

pub fn rand(seed: u32) -> f32 {
    hash(seed) as f32 / 4294967295.0
}

pub fn noise_grad(p: V2, seed: u32) -> f32 {
    // grid corners get a random vector
    // blend the vectors
    let i = vec2(1.0, 0.0);
    let j = vec2(0.0, 1.0);

    let p00 = vec2(p.x.floor(), p.y.floor());
    let p01 = p00 + j;
    let p10 = p00 + i;
    let p11 = p00 + i + j;

    let a00 = 2.0*PI*rand(seed + 1512347*p00.x as u32 + 213154127*p00.y as u32);
    let a01 = 2.0*PI*rand(seed + 1512347*p01.x as u32 + 213154127*p01.y as u32);
    let a10 = 2.0*PI*rand(seed + 1512347*p10.x as u32 + 213154127*p10.y as u32);
    let a11 = 2.0*PI*rand(seed + 1512347*p11.x as u32 + 213154127*p11.y as u32);

    let g00 = vec2(a00.cos(), a00.sin());
    let g01 = vec2(a01.cos(), a01.sin());
    let g10 = vec2(a10.cos(), a10.sin());
    let g11 = vec2(a11.cos(), a11.sin());

    
}