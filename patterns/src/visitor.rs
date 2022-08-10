use crate::animated_disc::AnimatedDisc;
use crate::disc::Disc;
use crate::dummy::Dummy;
use crate::dyn_dummy::DynDummy;
use crate::plane::Plane;

pub trait Visitor {
    fn visit_disc(&mut self, disc: &Disc);
    fn visit_animated_disc(&mut self, disc: &AnimatedDisc);
    fn visit_plane(&mut self, plane: &Plane);
    fn visit_dummy(&mut self, dummy: &Dummy);
    fn visit_dyn_dummy(&mut self, dummy: &DynDummy);
}

pub trait VisitorHandler {
    fn accept(&self, visitor: &mut dyn Visitor);
}
