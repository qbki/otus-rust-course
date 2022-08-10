use crate::common::{HitTest, Update};
use crate::ray::Ray;
use crate::dyn_dummy::DynDummyTrait;
use crate::visitor::{VisitorHandler, Visitor};
use crate::disc::Disc;

const ROTATION_SPEED: f32 = 0.2;

pub struct AnimatedDisc(pub Disc);

impl HitTest for AnimatedDisc {
    fn hit(&self, ray: &Ray) -> Option<char> {
        self.0.hit(ray)
    }
}

impl Update for AnimatedDisc {
    fn update(&mut self, t: f32) {
        let origin = self.0.origin;
        let length = origin.magnitude();
        let radians = origin.y.atan2(origin.x) + ROTATION_SPEED * t;
        self.0.origin.x = f32::cos(radians);
        self.0.origin.y = f32::sin(radians);
        self.0.origin *= length;
    }
}

impl VisitorHandler for AnimatedDisc {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_animated_disc(self);
    }
}

impl DynDummyTrait for AnimatedDisc {}
