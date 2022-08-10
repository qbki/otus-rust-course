use crate::common::{HitTest, Update};
use crate::ray::Ray;
use crate::visitor::{Visitor, VisitorHandler};

pub trait DynDummyTrait: HitTest + Update + VisitorHandler {}

pub struct DynDummy(pub Vec<Box<dyn DynDummyTrait>>);

impl HitTest for DynDummy {
    fn hit(&self, ray: &Ray) -> Option<char> {
        for item in self.0.iter() {
            let hit = item.hit(ray);
            if hit.is_some() {
                return hit;
            }
        }
        None
    }
}

impl Update for DynDummy {
    fn update(&mut self, t: f32) {
        for item in self.0.iter_mut() {
            item.update(t);
        }
    }
}

impl VisitorHandler for DynDummy {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_dyn_dummy(self);
    }
}
