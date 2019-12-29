use crate::math::{
    self,
    Point3,
    Vec3,
};
use crate::core::Ray;

pub struct CameraAxis {
    pub look_from: Point3,
    pub look_at: Point3,
}

struct CameraOrientation {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    orientation: CameraOrientation,
    lens_radius: f32,
}

pub struct FieldOfView {
    radians: f32,
}

impl FieldOfView {
    pub fn from_degrees(degrees: f32) -> FieldOfView {
        FieldOfView {
            radians: degrees.to_radians(),
        }
    }
    pub fn to_radians(&self) -> f32 {
        self.radians
    }
}

impl Camera {
    pub fn new(axis: CameraAxis, vector_up: Vec3, fov: FieldOfView, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        use math::InnerSpace;
        let theta = fov.to_radians();
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let origin = axis.look_from;
        let w = (axis.look_from - axis.look_at).normalize();
        let u = vector_up.cross(w).normalize();
        let v = w.cross(u);
        let orientation = CameraOrientation{ u, v, w };
        let lower_left_corner = origin
            - half_width  * focus_dist * orientation.u
            - half_height * focus_dist * orientation.v
            - focus_dist * orientation.w;
        Camera {
            origin,
            lower_left_corner,
            horizontal: 2.0 * half_width * focus_dist * orientation.u,
            vertical: 2.0 * half_height * focus_dist * orientation.v,
            lens_radius: aperture / 2.0,
            orientation,
        }
    }
    
    pub fn make_ray(&self, u: f32, v: f32) -> Ray {
        let point = self.lens_radius * random_point_in_unit_disk();
        let offset = point.x * self.orientation.u + point.y * self.orientation.v;
        let point_in_lens = self.origin + offset;
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset;
        Ray::new(point_in_lens, direction)
    }
}

fn random_point_in_unit_disk() -> Point3 {
    use crate::math::{vec3, EuclideanSpace, InnerSpace};
    use crate::random_float_from_0_to_1;
    loop {
        let mut vec = vec3(random_float_from_0_to_1(), random_float_from_0_to_1(), 0.0);
        vec = 2.0 * vec - vec3(1.0, 1.0, 0.0);
        if vec.magnitude() < 1.0 {
            return EuclideanSpace::from_vec(vec);
        }
    }
}