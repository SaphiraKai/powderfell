mod element;

use std::sync::Arc;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{uvec2, vec2};
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::texture::ImageSampler;
use bevy::window::PrimaryWindow;
use color_eyre::eyre::Report;
use element::*;
use pf_engine::element::Element;
use rand::prelude::*;

use pf_engine::{Playfield, Simulation, SimulationBufferHandle};

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

#[derive(Resource, Default)]
struct MousePosition(UVec2);

fn main() -> Result<(), Report> {
    let simulation = Simulation::new(WIDTH, HEIGHT);

    App::new()
        .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (get_mouse_coords, update).chain())
        // insert the playfield as a resource here
        .insert_resource(simulation)
        .init_resource::<MousePosition>()
        .run();

    Ok(())
}

fn update(
    mut simulation: ResMut<Simulation>,
    mut textures: ResMut<Assets<Image>>,
    simulation_buffer: Res<SimulationBufferHandle>,
    mouse_position: Res<MousePosition>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let element: Option<Arc<dyn Element>> = if mouse.pressed(MouseButton::Left) {
        Some(Sand::new())
    } else if mouse.pressed(MouseButton::Right) {
        Some(Water::new())
    } else {
        None
    };

    for _ in 0..8 {
        simulation.step();
        if let Some(e) = element.clone() {
            simulation.get_playfield_mut().spawn(
                e,
                mouse_position.0.x as usize,
                mouse_position.0.y as usize,
            );
        }
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

fn get_mouse_coords(
    sprite_query: Query<(&Sprite, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut mouse_position: ResMut<MousePosition>,
) {
    let (sprite, transform) = sprite_query.single();
    let translation = transform.compute_transform().translation.xy();
    let sprite_size = sprite.custom_size.unwrap();
    let min = translation - (sprite_size / 2.0);
    let max = translation + (sprite_size / 2.0);

    let powder_min = vec2(0.0, HEIGHT as f32);
    let powder_max = vec2(WIDTH as f32, 0.0);

    let Some(position) = windows.single().cursor_position() else {
        return;
    };

    let (camera, camera_transform) = camera.single();

    let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, position) else {
        return;
    };

    let powder_pos = map_range((min, max), (powder_min, powder_max), world_pos);
    let powder_pos = uvec2(powder_pos.x as _, powder_pos.y as _)
        .clamp(UVec2::ZERO, uvec2(WIDTH, HEIGHT) - UVec2::ONE);

    mouse_position.0 = powder_pos;
}

fn map_range(from_range: (Vec2, Vec2), to_range: (Vec2, Vec2), s: Vec2) -> Vec2 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
