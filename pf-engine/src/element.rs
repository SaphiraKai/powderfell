use std::any::Any;

use image::Rgba;

use crate::{Particle, Playfield};

pub trait Element: Any + Send + Sync + std::fmt::Debug {
    fn step(&self, particle: &mut Particle, playfield: &mut Playfield);
    fn name(&self) -> &str;
    fn color(&self) -> Rgba<u8>;
    fn as_any(&self) -> &dyn Any;
    fn density(&self) -> f32;
}
