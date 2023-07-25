use crate::vector::*;
use crate::game::*;

impl Game {
    
    pub fn cam_right(&self) -> Vec3 {
        let up = vec3(0.0, 1.0, 0.0);
        up.cross(self.cam_dir()).normalize()
    }

    pub fn cam_up(&self) -> Vec3 {
        self.cam_right().cross(self.cam_dir()).normalize() // see if it works without normalize
    }

    // pub fn cam_dir(&self) -> Vec3 {
    //     vec3(
    //         self.cam_polar_angle.sin() * self.cam_azimuthal_angle.cos(),
    //         self.cam_polar_angle.sin() * self.cam_azimuthal_angle.sin(),
    //         self.cam_polar_angle.cos(),
    //     )
    // }

        // note: self.azimuthal_angle
        // note: self.polar_angle
        // note: north pole 0,1,0
    pub fn cam_dir(&self) -> Vec3 {
        vec3(
            self.cam_polar_angle.sin() * self.cam_azimuthal_angle.cos(),
            self.cam_polar_angle.cos(),
            self.cam_polar_angle.sin() * self.cam_azimuthal_angle.sin(),
        )
    }
 
    pub fn turn_camera(&mut self, r: Vec2) {
        // let mut spherical = self.cam_dir().cartesian_to_spherical();
        // let r2 = r * 0.001;
        // spherical.y += r2.y;
        // spherical.z += r2.x;
        // spherical.x = 1.0;
        // self.cam_dir = spherical.spherical_to_cartesian().normalize();

        let mut r = r * 0.001;
        // r.y *= -1.0;
        self.cam_polar_angle -= r.y;
        self.cam_polar_angle = self.cam_polar_angle.max(0.0).min(PI);
        self.cam_azimuthal_angle += r.x;

        // let inclination = self.cam_dir.y.acos();    // theta
        // let azimuth = -self.cam_dir.z.atan2(self.cam_dir.x); // not sure if need the -  // phi
        // let sin_theta = inclination.sin();
        // let cos_theta = inclination.cos();
        // let sin_phi = azimuth.sin();
        // let cos_phi = azimuth.cos();    // cam_dir.y
        // let rot_spherical = [
        //     cos_phi, 0.0, -sin_phi,
        //     0.0, 1.0, 0.0,
        //     sin_phi, 0.0, cos_phi,
        // ];

    }
    
    pub fn movement(&mut self, dir: Vec3, dt: f32) {
        let speed = 1.0;

        // let cam_right = (up.cross(self.cam_dir)).normalize();
        // let cam_up = cam_right.cross(self.cam_dir).normalize();

        let cam_dir = self.cam_dir();
        let cam_dir = vec3(cam_dir.x, 0.0, cam_dir.z).normalize();
        let cam_right = self.cam_right();
        let cam_up = self.cam_up();
        let v = dir.z * cam_dir + dir.y * cam_up + dir.x * cam_right;

        // but cam_dir projected into xz plane

        // let v = self.cam_dir() * dir.dot(self.cam_dir()) + self.cam_right() * dir.dot(self.cam_right()) + self.cam_up() * dir.dot(self.cam_up());

        self.cam_pos += dt * speed * v;
    }
}