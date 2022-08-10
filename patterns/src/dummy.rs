use crate::common::{HitTest, Update};
use crate::ray::Ray;
use crate::disc::Disc;
use crate::animated_disc::AnimatedDisc;
use crate::plane::Plane;
use crate::visitor::{VisitorHandler, Visitor};

pub enum Object {
    Disc(Disc),
    AnimatedDisc(AnimatedDisc),
    Plane(Plane),
    #[allow(dead_code)]
    Dummy(Dummy),
}

pub struct Dummy(pub Vec<Object>);

impl HitTest for Dummy {
    fn hit(&self, ray: &Ray) -> Option<char> {
        for item in self.0.iter() {
            let hit = match item {
                Object::Disc(disc) => disc.hit(ray),
                Object::AnimatedDisc(disc) => disc.hit(ray),
                Object::Plane(plane) => plane.hit(ray),
                Object::Dummy(dummy) => dummy.hit(ray),
            };
            if hit.is_some() {
                return hit;
            }
        }
        None
    }
}

impl Update for Dummy {
    fn update(&mut self, t: f32) {
        for item in self.0.iter_mut() {
            match item {
                Object::Disc(disc) => disc.update(t),
                Object::AnimatedDisc(disc) => disc.update(t),
                Object::Plane(plane) => plane.update(t),
                Object::Dummy(dummy) => dummy.update(t),
            }
        }
    }
}

impl VisitorHandler for Dummy {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_dummy(self);
    }
}
