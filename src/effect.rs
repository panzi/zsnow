use std::{cmp::Ordering, f32::consts::TAU, time::Duration};

use crate::{color::Rgb, point3d::Point3D, rgb_image::RgbImage, size2d::Size2D};

pub trait Effect {
    fn animate(&mut self, frame: &mut RgbImage, frame_duration: Duration, current_time: Duration);

    fn change_amount(&mut self, _amount: i32) {}
}

#[derive(Debug, Clone)]
pub struct SnowEffect {
    particles: Vec<Particle>,
    draw_order: Vec<usize>,
    speed: f32,
    velocity: Point3D,
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
            speed: 16.0,
            angle: TAU * 15.0 / 180.0,
            depth: 1.0,
            variation: 0.25,
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
            draw_order: (0..options.particles).collect(),
            velocity: Point3D {
                x: options.angle.cos(),
                y: options.angle.sin(),
                z: 0.0,
            },
            speed: options.speed,
            depth: options.depth,
            variation: options.variation,
            color: options.color,
        }
    }
}

impl Effect for SnowEffect {
    fn change_amount(&mut self, amount: i32) {
        if amount < 0 {
            let amount = -amount as usize;
            if amount >= self.particles.len() {
                self.particles.clear();
            } else {
                self.particles.truncate(self.particles.len() - amount);
            }
            self.draw_order.truncate(self.particles.len());
        } else {
            // FIXME: adding particles doesn't work for some reason!
            self.particles.resize_with(self.particles.len() + amount as usize, Particle::default);
            self.draw_order.resize(self.particles.len(), 0);
        }

        for (i, draw_order) in self.draw_order.iter_mut().enumerate() {
            *draw_order = i;
        }
    }

    fn animate(&mut self, frame: &mut RgbImage, frame_duration: Duration, current_time: Duration) {
        let t = current_time.as_secs_f32() * self.speed;
        let dt = frame_duration.as_secs_f32() * self.speed;
        let width = frame.size().width as f32;
        let height = frame.size().height as f32;

        let mut i = 0;
        let n = self.particles.len();

        for p in &mut self.particles {
            if p.alive {
                p.position.x += p.velocity.x * dt;
                p.position.y += p.velocity.y * dt;
                p.position.z += p.velocity.z * dt;

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
                let seed = (i as f32 / (n - 1) as f32 * width * 1.317 + t) % 1.0;
                if t == 0.0 {
                    p.birth(seed, self.depth, self.variation, &self.velocity, frame.size());
                } else {
                    p.rebirth(seed, self.depth, self.variation, &self.velocity, frame.size());
                }
            }
            i += 1;
        }

        self.draw_order.sort_by(|&lhs, &rhs|
            self.particles[rhs].position.z.partial_cmp(&self.particles[lhs].position.z).unwrap_or(Ordering::Equal)
        );

        for &index in &self.draw_order {
            let p = &self.particles[index];

            if p.alive {
                frame.set_pixel_alpha(
                    p.position.x as usize,
                    p.position.y as usize,
                    self.color,
                    (255.0 - p.position.z * 255.0) as u8,
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

impl Particle {
    fn init_z(&mut self, seed: f32, depth: f32, velocity: &Point3D) {
        if velocity.z == 0.0 {
            // spawn randomly in depth
            self.position.z = ((seed * 123.0 + 31.45) % 1.0) * depth;
        } else if velocity.z > 0.0 {
            // spawn at near plane
            self.position.z = 0.0;
        } else {
            // spawn at far plane
            self.position.z = depth;
        }
    }

    fn init_velocity(&mut self, seed: f32, variation: f32, velocity: &Point3D) {
        let slowdown = 1.0 / self.position.z;

        let var_angle = (seed - 0.5) * variation * std::f32::consts::TAU;

        self.velocity = *velocity;
        let inv_var = 1.0 - variation;
        self.velocity.x = self.velocity.x * inv_var + var_angle.cos() * variation;
        self.velocity.y = self.velocity.y * inv_var + var_angle.sin() * variation;

        self.velocity *= slowdown;
    }

    pub fn birth(&mut self, seed: f32, depth: f32, variation: f32, velocity: &Point3D, screen: &Size2D) {
        let fwidth = screen.width as f32;
        let fheight = screen.height as f32;

        self.position.x = fwidth  * ((seed * 100.17) % 1.0);
        self.position.y = fheight * ((seed * 135.31) % 1.0);

        self.init_z(seed, depth, velocity);
        self.init_velocity(seed, variation, velocity);

        self.alive = true;
    }

    pub fn rebirth(&mut self, seed: f32, depth: f32, variation: f32, velocity: &Point3D, screen: &Size2D) {
        // scales sizes inversely to delta velocity
        let fwidth  = screen.width as f32 / velocity.x;
        let fheight = screen.height as f32 / velocity.y;
        let edge = fwidth + fheight;
        let spawn_pos = seed * edge;

        if velocity.x >= 0.0 {
            if velocity.y >= 0.0 {
                // spawn somewhere along left or top border
                if spawn_pos >= fwidth {
                    self.position.x = 0.0;
                    self.position.y = (spawn_pos - fwidth) * velocity.y;
                } else {
                    self.position.x = spawn_pos * velocity.x;
                    self.position.y = 0.0;
                }
            } else {
                // spawn somewhere along left or bottom border
                if spawn_pos >= fwidth {
                    self.position.x = 0.0;
                    self.position.y = (spawn_pos - fwidth) * velocity.y;
                } else {
                    self.position.x = spawn_pos * velocity.x;
                    self.position.y = fheight;
                }
            }
        } else {
            if velocity.y >= 0.0 {
                // spawn somewhere along right or top border
                if spawn_pos >= fwidth {
                    self.position.x = fwidth;
                    self.position.y = (spawn_pos - fwidth) * velocity.y;
                } else {
                    self.position.x = spawn_pos * velocity.x;
                    self.position.y = 0.0;
                }
            } else {
                // spawn somewhere along right or bottom border
                if spawn_pos >= fwidth {
                    self.position.x = fwidth;
                    self.position.y = (spawn_pos - fwidth) * velocity.y;
                } else {
                    self.position.x = spawn_pos * velocity.x;
                    self.position.y = fheight;
                }
            }
        }

        self.init_z(seed, depth, velocity);
        self.init_velocity(seed, variation, velocity);

        self.alive = true;
    }
}
