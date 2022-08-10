use nalgebra_glm::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub normal: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, normal: &Vec3) -> Ray {
        Ray { origin: origin.clone(), normal: normal.clone() }
    }
}
