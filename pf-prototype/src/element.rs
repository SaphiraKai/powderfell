use std::sync::Arc;

use image::{Pixel, Rgba};
use pf_engine::{element::Element, util::checked_move, Particle};
use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Debug)]
pub struct Sand;

impl Sand {
    pub fn new() -> Arc<Sand> {
        Arc::new(Sand)
    }
}

impl Element for Sand {
    fn step(&self, particle: &mut Particle, playfield: &mut pf_engine::Playfield) {
        let mut rng = thread_rng();

        let mut directions = [(0, 1), (1, 1), (-1, 1)];

        directions.shuffle(&mut rng);
        directions
            .iter()
            .any(|d| checked_move(particle, d.0, d.1, playfield));
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn color(&self) -> image::Rgba<u8> {
        let brightness: f32 = thread_rng().gen_range(0.8..1.0);

        Rgba::from([255, 233, 168, 255])
            .map_without_alpha(|c| (c as f32 * brightness).round() as u8)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn density(&self) -> f32 {
        1.607
    }
}

#[derive(Debug)]
pub struct Water;

impl Water {
    pub fn new() -> Arc<Water> {
        Arc::new(Water)
    }
}

impl Element for Water {
    fn step(&self, particle: &mut Particle, playfield: &mut pf_engine::Playfield) {
        const DISPERSION: i32 = 8;

        let mut rng = thread_rng();

        let mut random_move = |a: &mut [(i32, i32)]| {
            a.shuffle(&mut rng);
            a.iter()
                .any(|d| checked_move(particle, d.0, d.1, playfield))
        };

        if !random_move(&mut [(0, 1), (1, 1), (-1, 1)]) {
            random_move(&mut [(DISPERSION, 0), (-(DISPERSION), 0)]);
        }
    }

    fn name(&self) -> &str {
        todo!()
    }

    fn color(&self) -> image::Rgba<u8> {
        let brightness: f32 = thread_rng().gen_range(0.8..1.0);

        Rgba::from([33, 89, 255, 255]).map_without_alpha(|c| (c as f32 * brightness).round() as u8)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn density(&self) -> f32 {
        0.997
    }
}
