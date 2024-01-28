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

    // note: north pole 0,1,0
    pub fn cam_dir(&self) -> Vec3 {
        vec3(
            self.cam_polar_angle.sin() * self.cam_azimuthal_angle.cos(),
            self.cam_polar_angle.cos(),
            self.cam_polar_angle.sin() * self.cam_azimuthal_angle.sin(),
        )
    }
 
    pub fn turn_camera(&mut self, r: Vec2) {
        let r = r * 0.001;
        self.cam_polar_angle -= r.y;
        self.cam_polar_angle = self.cam_polar_angle.max(0.001).min(PI-0.001);
        self.cam_azimuthal_angle += r.x;
    }
    
    pub fn movement(&mut self, dir: Vec3, dt: f32) {
        let speed = 1.0;
        let cam_dir = self.cam_dir();
        let cam_dir = vec3(cam_dir.x, 0.0, cam_dir.z).normalize();  // project into xz plane
        let cam_right = self.cam_right();
        let cam_up = self.cam_up();
        let v = dir.z * cam_dir + dir.y * cam_up + dir.x * cam_right;

        self.cam_pos += dt * speed * v;
    }
}