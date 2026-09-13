use std::{f32::consts::PI, time::Duration};

use crate::{color::Rgb, rgb_image::RgbImage};

pub trait Effect {
    fn animate(&mut self, frame: &mut RgbImage, time: Duration);
}

#[derive(Debug, Clone)]
pub struct SnowEffect {
    particles: Vec<Particle>,
    speed: f32,
    dy: f32,
    dx: f32,
    depth: f32,
    variation: f32,
    color: Rgb,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnowOptions {
    pub particles: usize,
    pub speed: f32,
    pub angle: f32,
    pub depth: f32,
    pub variation: f32,
    pub color: Rgb,
}

impl SnowOptions {
    #[inline]
    pub fn new() -> Self {
        Self {
            particles: 32,
            speed: 1.0,
            angle: PI * -0.35,
            depth: 0.5,
            variation: 0.02,
            color: Rgb::from_u32(0xFFFFFF),
        }
    }

    #[inline]
    pub fn particles(&mut self, value: usize) -> &mut Self {
        self.particles = value;
        self
    }

    #[inline]
    pub fn speed(&mut self, value: f32) -> &mut Self {
        self.speed = value;
        self
    }

    #[inline]
    pub fn angle(&mut self, value: f32) -> &mut Self {
        self.angle = value;
        self
    }

    #[inline]
    pub fn depth(&mut self, value: f32) -> &mut Self {
        self.depth = value;
        self
    }

    #[inline]
    pub fn variation(&mut self, value: f32) -> &mut Self {
        self.variation = value;
        self
    }

    #[inline]
    pub fn color(&mut self, value: Rgb) -> &mut Self {
        self.color = value;
        self
    }

    #[inline]
    pub fn build(&self) -> SnowEffect {
        SnowEffect::new(self)
    }
}

impl Default for SnowOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl SnowEffect {
    #[inline]
    pub fn new(options: &SnowOptions) -> Self {
        Self {
            particles: vec![Particle::default(); options.particles],
            dy: options.angle.sin(),
            dx: options.angle.cos(),
            speed: options.speed,
            depth: options.depth,
            variation: options.variation,
            color: options.color,
        }
    }
}

impl Effect for SnowEffect {
    fn animate(&mut self, frame: &mut RgbImage, time: Duration) {
        let t = time.as_secs_f32() * self.speed;
        let width = frame.size().width as f32;
        let height = frame.size().height as f32;

        for p in &mut self.particles {
            if p.alive {
                p.position.x += p.velocity.x * t;
                p.position.y += p.velocity.y * t;
                p.position.z += p.velocity.z * t;

                if p.velocity.x >= 0.0 {
                    if p.position.x >= width {
                        p.alive = false;
                    }
                } else {
                    if p.position.x < 0.0 {
                        p.alive = false;
                    }
                }

                if p.velocity.y >= 0.0 {
                    if p.position.y >= height {
                        p.alive = false;
                    }
                } else {
                    if p.position.y < 0.0 {
                        p.alive = false;
                    }
                }
            } else {
                p.alive = true;
                p.position.y = 0.0;
                p.position.x = width * ((t * 10.0) % 1.0);
                let z = self.depth * ((t * 50.0) % 1.0);
                let slowdown = 1.0 - z;
                p.position.z = z * 255.0;
                p.velocity.x = self.dx * slowdown * (self.variation * ((t * 30.0) % 1.0));
                p.velocity.y = self.dy * slowdown * (self.variation * ((t * 30.0 + 5.0) % 1.0));
                p.velocity.z = 0.0;
            }

            if p.alive {
                frame.set_pixel_alpha(
                    p.position.x as usize,
                    p.position.y as usize,
                    self.color,
                    (255.0 - p.position.z) as u8,
                );
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Particle {
    pub position: Point3D,
    pub velocity: Point3D,
    pub alive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}
