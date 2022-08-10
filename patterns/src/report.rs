use crate::animated_disc::AnimatedDisc;
use crate::disc::Disc;
use crate::dummy::{Dummy, Object};
use crate::dyn_dummy::DynDummy;
use crate::plane::Plane;
use crate::visitor::{Visitor, VisitorHandler};

pub struct Report {
    text: String,
}

impl Report {
    pub fn new() -> Self {
        Report {
            text: String::new(),
        }
    }

    pub fn print(&self) {
        print!("{}", self.text);
    }
}

impl Visitor for Report {
    fn visit_disc(&mut self, disc: &Disc) {
        self.text += format!(
            "Disc ({}, {}, {})\n",
            disc.origin.x, disc.origin.y, disc.origin.z
        )
        .as_str();
    }

    fn visit_animated_disc(&mut self, disc: &AnimatedDisc) {
        self.text += format!(
            "Disc ({}, {}, {})\n",
            disc.0.origin.x, disc.0.origin.y, disc.0.origin.z
        )
        .as_str();
    }

    fn visit_plane(&mut self, plane: &Plane) {
        self.text += format!(
            "Plane ({}, {}, {})\n",
            plane.origin.x, plane.origin.y, plane.origin.z
        )
        .as_str();
    }

    fn visit_dummy(&mut self, dummy: &Dummy) {
        self.text += "Dummy (no origin)\n";
        for item in dummy.0.iter() {
            match item {
                Object::Disc(disc) => disc.accept(self),
                Object::AnimatedDisc(disc) => disc.accept(self),
                Object::Plane(plane) => plane.accept(self),
                Object::Dummy(dummy) => dummy.accept(self),
                Object::DynDummy(dummy) => dummy.accept(self),
            }
        }
    }

    fn visit_dyn_dummy(&mut self, dummy: &DynDummy) {
        self.text += "DynDummy (no origin)\n";
        for item in dummy.0.iter() {
            item.accept(self);
        }
    }
}
