use std::sync::{Arc, Mutex, OnceLock};

use bevy::asset::Handle;
use bevy::ecs::system::Resource;
use bevy::render::texture::Image;
use bevy::utils::info;
use element::Element;
use image::{Rgba, RgbaImage};

pub mod element;
pub mod util;

#[derive(Clone, Debug)]
pub struct Particle {
    element: Arc<dyn Element>,
    color: Rgba<u8>,
    id: u64,
    x: usize,
    y: usize,
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

static PARTICLE_ID: OnceLock<Mutex<u64>> = OnceLock::new();

impl Particle {
    fn step(&mut self, playfield: &mut Playfield) {
        self.element.clone().step(self, playfield);
    }

    pub fn update(&self, playfield: &mut Playfield) {
        playfield.despawn(self.x, self.y);
        playfield.spawn(self.element.clone(), self.x, self.y);
    }

    pub fn new(element: Arc<dyn Element>, x: usize, y: usize) -> Particle {
        let mut id_generator = PARTICLE_ID.get_or_init(|| Mutex::new(0)).lock().unwrap();

        let id = *id_generator;
        *id_generator += 1;

        Particle {
            element: element.clone(),
            color: element.color(),
            id,
            x,
            y,
        }
    }

    fn move_to(&self, x: usize, y: usize, playfield: &mut Playfield) -> bool {
        if playfield.get(self.x, self.y) == Some(self) {
            playfield.despawn(self.x, self.y);
        }

        playfield.spawn(self.element.clone(), x, y)
    }

    fn swap_with(&self, other: &Particle, playfield: &mut Playfield) {
        playfield.despawn(self.x, self.y);
        playfield.despawn(other.x, other.y);

        playfield.spawn(self.element.clone(), other.x, other.y);
        playfield.spawn(other.element.clone(), self.x, self.y);
    }
}

#[derive(Clone, Debug)]
pub struct Playfield {
    height: u32,
    width: u32,
    data: Vec<Vec<Option<Particle>>>,
}

impl Playfield {
    pub fn new(height: u32, width: u32) -> Playfield {
        let data = vec![vec![None; width as usize]; height as usize];

        Playfield {
            height,
            width,
            data,
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Particle> {
        self.data.get(y)?.get(x)?.as_ref()
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Particle> {
        self.data.get_mut(y)?.get_mut(x)?.as_mut()
    }

    pub fn spawn(&mut self, element: Arc<dyn Element>, x: usize, y: usize) -> bool {
        let exists = self.data[y][x].is_some();

        self.data[y][x] = Some(Particle::new(element, x, y));

        exists
    }

    pub fn despawn(&mut self, x: usize, y: usize) {
        self.data[y][x] = None;
    }

    pub fn as_image(&self) -> RgbaImage {
        RgbaImage::from_vec(
            self.width,
            self.height,
            self.data
                .clone()
                .into_iter()
                .flatten()
                .flat_map(|o| o.map(|p| p.color.0).unwrap_or([0; 4]))
                .collect(),
        )
        .unwrap()
    }
}

#[derive(Resource)]
pub struct SimulationBufferHandle(pub Handle<Image>);

#[derive(Clone, Debug, Resource)]
pub struct Simulation {
    playfield: Playfield,
}

impl Simulation {
    pub fn new(height: u32, width: u32) -> Simulation {
        Simulation {
            playfield: Playfield::new(height, width),
        }
    }

    pub fn get_playfield(&self) -> &Playfield {
        &self.playfield
    }

    pub fn get_playfield_mut(&mut self) -> &mut Playfield {
        &mut self.playfield
    }

    pub fn step(&mut self) {
        for y in 0..self.playfield.height as usize {
            for x in 0..self.playfield.width as usize {
                let (x, y) = (
                    (-(x as i32) + self.playfield.width as i32) as usize - 1,
                    (-(y as i32) + self.playfield.height as i32) as usize - 1,
                );

                let particle = self.playfield.get(x, y).cloned();

                if let Some(mut p) = particle {
                    p.step(&mut self.playfield);
                }
            }
        }
    }
}
