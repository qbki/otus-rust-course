mod animated_disc;
mod common;
mod disc;
mod dummy;
mod plane;
mod ray;
mod report;
mod visitor;

use animated_disc::AnimatedDisc;
use common::{App, Update, HitTest};
use disc::Disc;
use dummy::{Object, Dummy};
use nalgebra_glm::vec3;
use plane::Plane;
use ray::Ray;
use report::Report;
use std::time;
use visitor::Visitor;

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
        let elapsed_result = now.duration_since(last);
        if let Ok(elapsed) = elapsed_result {
            print!("\x1B[2J"); // clear screen
            print!("\x1B[H"); // move cursor to (0, 0)
            app.update(&mut root, elapsed.as_secs_f32());
            app.render(&root);
            last = now;
        }

        let mut report = Report::new();
        report.visit_dummy(&mut root);
        report.print();
    }
}
