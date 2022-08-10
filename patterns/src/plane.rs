use nalgebra_glm::Vec3;
use crate::common::{HitTest, Update};
use crate::ray::Ray;
use crate::visitor::{VisitorHandler, Visitor};

pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
    pub symbol: char,
}

impl HitTest for Plane {
    fn hit(&self, ray: &Ray) -> Option<char> {
        let dot_product = self.normal.dot(&ray.normal);
        let has_hit = if dot_product.abs() > f32::EPSILON {
            let t = (self.origin - ray.origin).dot(&self.normal) / dot_product;
            t > 0.0
        } else {
            false
        };
        if has_hit {
            Some(self.symbol)
        } else {
            None
        }
    }
}

impl Update for Plane {
    fn update(&mut self, _: f32) {}
}

impl VisitorHandler for Plane {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_plane(self);
    }
}
