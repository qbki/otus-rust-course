use nalgebra_glm::Vec3;
use crate::common::{HitTest, Update};
use crate::ray::Ray;
use crate::visitor::{VisitorHandler, Visitor};

pub struct Disc {
    pub origin: Vec3,
    pub normal: Vec3,
    pub radius: f32,
    pub symbol: char,
}

impl HitTest for Disc {
    fn hit(&self, ray: &Ray) -> Option<char> {
        let dot_product = self.normal.dot(&ray.normal);
        let has_hit = if dot_product.abs() > f32::EPSILON {
            let distance = (self.origin - ray.origin).dot(&self.normal) / dot_product;
            let intersection_point = ray.origin + ray.normal * distance;
            let length = (self.origin - intersection_point).magnitude();
            length <= self.radius
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

impl Update for Disc {
    fn update(&mut self, _: f32) {}
}

impl VisitorHandler for Disc {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_disc(self);
    }
}
