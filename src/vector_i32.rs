use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub fn vec3i(x: i32, y: i32, z: i32) -> Vec3i { Vec3i {x, y, z} }

impl std::ops::Add<Vec3i> for Vec3i {
    type Output = Vec3i;

    fn add(self, _rhs: Vec3i) -> Vec3i {
        Vec3i { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z}
    }
}

impl PartialOrd for Vec3i {
    fn partial_cmp(&self, other: &Vec3i) -> Option<Ordering> {
        // Compare the x component
        let x_cmp = self.x.partial_cmp(&other.x);

        // If the x components are not equal, return the comparison result
        if x_cmp != Some(Ordering::Equal) {
            return x_cmp;
        }

        // If the x components are equal, compare the y component
        let y_cmp = self.y.partial_cmp(&other.y);

        // If the y components are not equal, return the comparison result
        if y_cmp != Some(Ordering::Equal) {
            return y_cmp;
        }

        // If the y components are equal, compare the z component
        self.z.partial_cmp(&other.z)
    }
}