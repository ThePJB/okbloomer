use crate::random::*;
use crate::vector::*;

pub fn heightmap(p: Vec2, seed: u32) -> f32 {
    let h = heightmap_unit(p, seed);
    h.max(0.0) * 30.0
}

// iq would do rotmap on baby p for less artifacts
pub fn heightmap_unit(p: Vec2, seed: u32) -> f32 {
    let f = 8.0;
    1.000 * noise_grad(p * f * 1.000, seed.wrapping_add(1713513437)) +
    0.500 * noise_grad(p * f * 2.000, seed.wrapping_add(1967234473)) +
    0.250 * noise_grad(p * f * 4.000, seed.wrapping_add(3851234713)) +
    0.125 * noise_grad(p * f * 8.000, seed.wrapping_add(3572312267)) /
    1.875 + 0.5
}

#[test]
pub fn hm_test() {
    use crate::kimg::*;

    let w = 1000;
    let h = 1000;
    let mut imbuf = ImageBuffer::new(w, h);
    for i in 0..w {
        for j in 0..h {
            let p = vec2(i as f32 / w as f32, j as f32 / h as f32);
            let h = heightmap_unit(p, 69);
            let c = ((255.0 * h) as u8, (255.0 * h) as u8, (255.0 * h) as u8);
            imbuf.set_px(i,j,c);
        }
    }
    imbuf.dump_to_file("hm.png");
}