use std::{cmp::Ordering, f32::consts::PI, time::Duration};

use crate::{color::Rgb, point3d::Point3D, rgb_image::RgbImage, size2d::Size2D, xorshift::XOrShift32};

pub trait Effect {
    fn animate(&mut self, screen: &Size2D, frame_duration: Duration, current_time: Duration);
    fn draw(&self, frame: &mut RgbImage);

    fn change_amount(&mut self, _amount: i32) -> u32 { 0 }
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
    rand: XOrShift32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnowOptions {
    pub particles: usize,
    pub speed: f32,
    pub velocity: Point3D,
    pub depth: f32,
    pub variation: f32,
    pub color: Rgb,
    pub seed: u32,
}

/// some large-ish prime
pub const DEFAULT_RANDOM_SEED: u32 = 999999937;
pub const DEFAULT_VELOCITY: Point3D = Point3D {
    x: 0.8660254037844387,
    y: 0.49999999999999994,
    z: 0.0,
};

impl SnowOptions {
    #[inline]
    pub fn new() -> Self {
        let angle = PI * 30.0 / 180.0;
        Self {
            particles: 32,
            speed: 16.0,
            velocity: Point3D {
                x: angle.cos(),
                y: angle.sin(),
                z: 0.0,
            },
            depth: 1.0,
            variation: 0.25,
            color: Rgb::from_u32(0xFFFFFF),
            seed: DEFAULT_RANDOM_SEED,
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
    pub fn seed(&mut self, value: u32) -> &mut Self {
        self.seed = value;
        self
    }

    #[inline]
    pub fn velocity(&mut self, value: &Point3D) -> &mut Self {
        self.velocity = *value;
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
            velocity: options.velocity,
            speed: options.speed,
            depth: options.depth,
            variation: options.variation,
            color: options.color,
            rand: XOrShift32::new(options.seed),
        }
    }

    #[inline]
    pub fn set_angle_grad(&mut self, angle: f32) {
        self.set_angle_rad(angle * PI / 180.0);
    }

    #[inline]
    pub fn set_angle_rad(&mut self, angle: f32) {
        self.velocity.x = angle.cos();
        self.velocity.y = angle.sin();
    }
}

impl Effect for SnowEffect {
    fn change_amount(&mut self, amount: i32) -> u32 {
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

        self.particles.len() as u32
    }

    fn animate(&mut self, screen: &Size2D, frame_duration: Duration, current_time: Duration) {
        let t = current_time.as_secs_f32() * self.speed;
        let dt = frame_duration.as_secs_f32() * self.speed;
        let width = screen.width as f32;
        let height = screen.height as f32;

        //let angle = (t * 0.01) % std::f32::consts::TAU;
        //self.velocity.x = angle.cos();
        //self.velocity.y = angle.sin();

        for p in self.particles.iter_mut() {
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
            }

            if !p.alive {
                let seed = self.rand.next_f32();
                if t == 0.0 {
                    p.birth(seed, self.depth, self.variation, &self.velocity, screen);
                } else {
                    p.rebirth(seed, self.depth, self.variation, &self.velocity, screen);
                }
            }
        }

        self.draw_order.sort_by(|&lhs, &rhs|
            self.particles[rhs].position.z.partial_cmp(&self.particles[lhs].position.z).unwrap_or(Ordering::Equal)
        );
    }

    fn draw(&self, frame: &mut RgbImage) {
        for &index in &self.draw_order {
            let p = &self.particles[index];

            if p.alive {
                let Point3D { x, y, .. } = p.position;

                if x >= 0.0 && y >= 0.0 {
                    frame.set_pixel_alpha(
                        x as usize,
                        y as usize,
                        self.color,
                        (255.0 - p.position.z * 255.0) as u8,
                    );
                }
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

        let var_angle = ((seed * 1.1731) % 1.0 - 0.5) * variation * std::f32::consts::PI;

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
        // TODO: move things out of loop
        let vx = velocity.x;
        let vy = velocity.y;

        if vx == 0.0 {
            self.position.x = seed * screen.width as f32;
            self.position.y = if vy >= 0.0 { 0.0 } else { screen.height as f32 - 1.0 };
        } else if vy == 0.0 {
            self.position.x = if vx >= 0.0 { 0.0 } else { screen.width as f32 - 1.0 };
            self.position.y = seed * screen.height as f32;
        } else {
            let border: f32 = 8.0;
            let ext_width = border + screen.width as f32;// * spawn_dy;
            let virt_width = ext_width + screen.height as f32;// * spawn_dx;
            let area = (ext_width as f32 + virt_width as f32) * (border as f32);

            let spawn_pixel = area * seed;

            // within the virtual area
            let mut spawn_y = (spawn_pixel / virt_width).floor();
            let mut spawn_x = (spawn_pixel % virt_width).floor();

            // mapping to seperate horizontal/vertical areas
            if spawn_x >= ext_width {
                // in vertical area
                //(spawn_y as f32 / spawn_dy, ((spawn_x - ext_width) as f32) / spawn_dx)
                let x = spawn_y;
                spawn_y = spawn_x - ext_width;
                spawn_x = x;

                if vx >= 0.0 {
                    spawn_x -= border;
                } else {
                    spawn_x += screen.width as f32;
                }
            } else {
                if vy >= 0.0 {
                    spawn_y -= border;
                } else {
                    spawn_y += screen.height as f32;
                }
            }

            self.position.x = spawn_x;
            self.position.y = spawn_y;
        }

        self.init_z(seed, depth, velocity);
        self.init_velocity(seed, variation, velocity);

        self.alive = true;
    }
}
