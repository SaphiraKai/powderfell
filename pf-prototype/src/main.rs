mod element;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::ImageSampler;
use color_eyre::eyre::Report;
use element::*;
use rand::prelude::*;

use pf_engine::{Simulation, SimulationBufferHandle};

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

fn main() -> Result<(), Report> {
    let simulation = Simulation::new(WIDTH, HEIGHT);

    App::new()
        .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        // insert the playfield as a resource here
        .insert_resource(simulation)
        .run();

    Ok(())
}

fn update(
    mut simulation: ResMut<Simulation>,
    mut textures: ResMut<Assets<Image>>,
    simulation_buffer: Res<SimulationBufferHandle>,
) {
    for _ in 0..8 {
        simulation.step();
        simulation.get_playfield_mut().spawn(Sand::new(), 96, 16);
        simulation.get_playfield_mut().spawn(Water::new(), 32, 16);
    }

    // update playfield code here
    let binding = simulation.get_playfield_mut().as_image();
    let image_data = binding.as_raw();

    let image = textures.get_mut(simulation_buffer.0.clone()).unwrap();
    // image.data = create_image_data(image_data);
    image.data = image_data.clone();
}

fn setup(mut commands: Commands, mut textures: ResMut<Assets<Image>>) {
    let mut image = Image::new(
        Extent3d {
            width: WIDTH,
            height: HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all(),
    );
    image.sampler = ImageSampler::nearest();

    let render_texture = textures.add(image);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(vec2(WIDTH as f32, HEIGHT as f32) * 3.),
            ..default()
        },
        texture: render_texture.clone(),
        ..default()
    });
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(SimulationBufferHandle(render_texture));
}
