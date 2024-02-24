use image::GrayImage;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    element: u8,
    x: usize,
    y: usize,
}

impl Particle {
    fn step(&mut self, playfield: &mut Playfield) {
        if self.element == 0 {
            return;
        }

        if self.y + 1 < playfield.height as usize {
            let down = playfield.get(self.x, self.y + 1);

            if down.is_none() {
                self.move_to(self.x, self.y + 1, playfield);
            }
        }
    }

    pub fn update(&self, playfield: &mut Playfield) {
        self.despawn(playfield);
        playfield.spawn(self.element, self.x, self.y);
    }

    pub fn new(element: u8, x: usize, y: usize) -> Particle {
        Particle { element, x, y }
    }

    fn despawn(&self, playfield: &mut Playfield) {
        playfield.spawn(0, self.x, self.y);
    }

    fn move_to(&self, x: usize, y: usize, playfield: &mut Playfield) {
        self.despawn(playfield);
        playfield.spawn(self.element, x, y);
    }

    fn swap_with(&self, other: &Particle, playfield: &mut Playfield) {
        self.despawn(playfield);
        other.despawn(playfield);

        playfield.spawn(self.element, other.x, other.y);
        playfield.spawn(other.element, self.x, self.y);
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

    pub fn spawn(&mut self, element: u8, x: usize, y: usize) {
        self.data[y][x] = Some(Particle::new(element, x, y));
    }

    pub fn as_image(&self) -> GrayImage {
        GrayImage::from_vec(
            self.width,
            self.height,
            self.data
                .clone()
                .into_iter()
                .flatten()
                .map(|o| o.map(|p| p.element).unwrap_or(0))
                .collect(),
        )
        .unwrap()
    }
}

#[derive(Clone, Debug)]
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

                let particle = self.playfield.get(x, y).copied();

                if let Some(mut p) = particle {
                    p.step(&mut self.playfield);
                }
            }
        }
    }
}
