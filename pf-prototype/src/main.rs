use std::{thread, time::Duration};

use color_eyre::eyre::Report;
use rand::prelude::*;
use show_image::{create_window, ImageInfo, ImageView};

use pf_engine::Simulation;

#[show_image::main]
fn main() -> Result<(), Report> {
    let mut simulation = Simulation::new(64, 64);

    simulation.get_playfield_mut().spawn(255, 32, 0);
    simulation.get_playfield_mut().spawn(255, 32, 2);

    let window = create_window("image", Default::default())?;

    let mut rng = thread_rng();

    let mut i = 0;
    loop {
        // if i % 5 == 0 {
        //     simulation
        //         .get_playfield_mut()
        //         .spawn(255, rng.gen_range(0..32) % 32, 0);
        // }

        thread::sleep(Duration::from_secs_f32(1.0 / 60.0));

        let playfield = simulation.get_playfield();

        let binding = playfield.as_image();
        let image = ImageView::new(
            ImageInfo::mono8(playfield.width(), playfield.height()),
            binding.as_raw(),
        );

        window.set_image("frame", image)?;

        simulation.step();

        i += 1;
    }
}
