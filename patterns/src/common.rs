use crate::Ray;

pub struct App {
    pub distance: f32,
    pub width: i32,
    pub height: i32,
}

pub trait HitTest {
    fn hit(&self, ray: &Ray) -> Option<char>;
}

pub trait Update {
    fn update(&mut self, t: f32);
}

