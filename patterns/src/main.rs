use std::time;
use nalgebra_glm::{Vec3, vec3};

const ROTATION_SPEED: f32 = 0.2;

struct App {
    distance: f32,
    width: i32,
    height: i32,
}

struct Ray {
    origin: Vec3,
    normal: Vec3,
}

struct Plane {
    origin: Vec3,
    normal: Vec3,
    symbol: char,
}

struct Disc {
    origin: Vec3,
    normal: Vec3,
    radius: f32,
    symbol: char,
}

struct AnimatedDisc(Disc);

enum Object {
    Disc(Disc),
    AnimatedDisc(AnimatedDisc),
    Plane(Plane),
    #[allow(dead_code)]
    Dummy(Dummy),
}

struct Dummy(Vec<Object>);

trait HitTest {
    fn hit(&self, ray: &Ray) -> Option<char>;
}

trait Update {
    fn update(&mut self, t: f32);
}

trait Visitor {
    fn visit_disc(&mut self, disc: &Disc);
    fn visit_animated_disc(&mut self, disc: &AnimatedDisc);
    fn visit_plane(&mut self, plane: &Plane);
    fn visit_dummy(&mut self, dummy: &Dummy);
}

trait VisitorHandler {
    fn accept(&self, visitor: &mut dyn Visitor);
}

impl Ray {
    fn new(origin: &Vec3, normal: &Vec3) -> Ray {
        Ray { origin: origin.clone(), normal: normal.clone() }
    }
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

impl HitTest for AnimatedDisc {
    fn hit(&self, ray: &Ray) -> Option<char> {
        self.0.hit(ray)
    }
}

impl HitTest for Dummy {
    fn hit(&self, ray: &Ray) -> Option<char> {
        for item in self.0.iter() {
            let hit = match item {
                Object::Disc(disc) => disc.hit(ray),
                Object::AnimatedDisc(disc) => disc.hit(ray),
                Object::Plane(plane) => plane.hit(ray),
                Object::Dummy(plane) => plane.hit(ray),
            };
            if hit.is_some() {
                return hit;
            }
        }
        None
    }
}

impl Update for Plane {
    fn update(&mut self, _: f32) {}
}

impl Update for Disc {
    fn update(&mut self, _: f32) {}
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

impl App {
    fn update(&self, obj: &mut Dummy, t: f32) {
        obj.update(t);
    }

    fn render(&self, dummy: &Dummy) {
        let aspect_ratio = self.width as f32 / (self.height as f32 * 2.0);
        let half_aspect_ratio = aspect_ratio / 2.0;
        let mut frame_buffer = vec![vec![' '; self.width as usize]; self.height as usize];
        let ray_origin = vec3(0.0, 0.0, -self.distance);
        for screen_x in 0..self.width {
            for screen_y in 0..self.height {
                let x = (screen_x as f32 / self.width as f32) * aspect_ratio - half_aspect_ratio;
                let y = (screen_y as f32 / self.height as f32) - 0.5;
                let ray_normal = (vec3(x, y, 0.0) - ray_origin).normalize();
                let ray = Ray::new(&ray_origin, &ray_normal);
                if let Some(symbol) = dummy.hit(&ray) {
                    frame_buffer[screen_y as usize][screen_x as usize] = symbol;
                }
            }
        }
        for row in frame_buffer.into_iter().rev() {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
    }
}

impl VisitorHandler for Disc {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_disc(self);
    }
}

impl VisitorHandler for AnimatedDisc {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_animated_disc(self);
    }
}

impl VisitorHandler for Plane {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_plane(self);
    }
}

impl VisitorHandler for Dummy {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_dummy(self);
    }
}

struct Report {
    text: String,
}

impl Report {
    fn new() -> Self {
        Report { text: String::new() }
    }

    fn print(&self) {
        print!("{}", self.text);
    }
}

impl Visitor for Report {
    fn visit_disc(&mut self, disc: &Disc) {
        self.text += format!("Disc ({}, {}, {})\n", disc.origin.x, disc.origin.y, disc.origin.z).as_str();
    }

    fn visit_animated_disc(&mut self, disc: &AnimatedDisc) {
        self.text += format!("Disc ({}, {}, {})\n", disc.0.origin.x, disc.0.origin.y, disc.0.origin.z).as_str();
    }

    fn visit_plane(&mut self, plane: &Plane) {
        self.text += format!("Plane ({}, {}, {})\n", plane.origin.x, plane.origin.y, plane.origin.z).as_str();
    }

    fn visit_dummy(&mut self, dummy: &Dummy) {
        self.text += "Dummy (no origin)\n";
        for item in dummy.0.iter() {
            match item {
                Object::Disc(disc) => disc.accept(self),
                Object::AnimatedDisc(disc) => disc.accept(self),
                Object::Plane(plane) => plane.accept(self),
                Object::Dummy(dummy) => dummy.accept(self),
            }
        }
    }
}

fn main() {
    let app = App {
        width: 80,
        height: 30,
        distance: 2.0,
    };

    let mut root = Dummy(vec![
        Object::AnimatedDisc(AnimatedDisc(Disc { origin: vec3(0.5, 0.0, 0.0), normal: vec3(0.0, 0.0, -1.0), radius: 0.1, symbol: '*' })),
        Object::AnimatedDisc(AnimatedDisc(Disc { origin: vec3(-0.5, 0.0, 0.0), normal: vec3(0.0, 0.0, -1.0), radius: 0.1, symbol: '*' })),
        Object::Disc(Disc { origin: vec3(0.0, 0.0, 0.0), normal: vec3(0.0, 0.0, -1.0), radius: 0.1, symbol: '*' }),
        Object::Plane(Plane { origin: vec3(0.0, 10.0, 0.0), normal: vec3(0.0, -1.0, 0.0), symbol: '.' }),
        Object::Plane(Plane { origin: vec3(0.0, -10.0, 0.0), normal: vec3(0.0, 1.0, 0.0), symbol: '~' }),
    ]);

    let mut last = time::SystemTime::now();

    loop {
        let now = time::SystemTime::now();
        let mut report = Report::new();
        let elapsed_result = now.duration_since(last);
        if let Ok(elapsed) = elapsed_result {
            print!("\x1B[2J"); // clear screen
            print!("\x1B[H"); // move cursor to (0, 0)
            app.update(&mut root, elapsed.as_secs_f32());
            app.render(&root);
            last = now;
        }
        report.visit_dummy(&mut root);
        report.print();
    }
}
