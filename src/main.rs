use std::f32::consts::PI;

use bevy::{prelude::*, asset::Asset, render::render_resource::TextureDimension};
use bevy_egui::{egui, EguiContext, EguiPlugin};



use rand::prelude::*;

mod debug_texture;

use debug_texture::uv_debug_texture;

#[derive(Component)]
struct Shape;


#[derive(Resource)]
struct ColorF32 {
    r: f32,
    g: f32,
    b: f32
}

impl Default for ColorF32 {
    fn default() -> Self {
        Self {
            r: 1., g: 1., b: 1.
        }
    }
}

#[derive(Resource)]
#[derive(Default)]
struct ProgramState {
    color_sliders: ColorF32
}

const X_EXTENT: f32 = 14.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_system(ui_example)
        .add_system(rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) { 
    commands.insert_resource(ProgramState::default());

    let debug_material = materials.add(StandardMaterial {
        //base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let shapes = [
        meshes.add(shape::Cube::default().into())
    ];
    let shapes_len = shapes.len();
    for (i, shape) in shapes.into_iter().enumerate() {
        let pbr = PbrBundle {
            mesh: shape,
            material: debug_material.clone(),
            transform: Transform::from_xyz(
                0.,
                2.,
                5.
            ).with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
        };
        commands.spawn((pbr, Shape));
    }

    let point_light = PointLight {
        intensity: 9000.,
        range: 100.,
        shadows_enabled: true,
        ..default()
    };

    let point_light_bundle = PointLightBundle {
        point_light,
        transform: Transform::from_xyz(8., 16., 8.),
        ..default()
    };

    commands.spawn(point_light_bundle);

    let camera_3d_bundle = Camera3dBundle {
        transform: Transform::from_xyz(0., 6., 12.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    };

    commands.spawn(camera_3d_bundle);
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

fn ui_example(mut egui_context: ResMut<EguiContext>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut program_state: ResMut<ProgramState>) {

    let mut update_material_color =  |materials: &mut ResMut<Assets<StandardMaterial>>, program_state: &mut ResMut<ProgramState>| {
        for (_id, material) in materials.iter_mut() {
            material.base_color = Color::rgb_linear(program_state.color_sliders.r,
            program_state.color_sliders.g,
            program_state.color_sliders.b);
        }
    };

    egui::Window::new("Gay Agenda").show(egui_context.ctx_mut(), |ui| {
        if ui.button("GAY").clicked() {
            println!("ACTIVATE GAY");
            
            program_state.color_sliders.r = rand::thread_rng().gen();
            program_state.color_sliders.g = rand::thread_rng().gen();
            program_state.color_sliders.b = rand::thread_rng().gen();

            update_material_color(&mut materials, &mut program_state);
        }

        ui.horizontal(|ui| {
            ui.label("R");
            if ui.add(egui::Slider::new(&mut program_state.color_sliders.r, 0.0..=1.0)).changed() {
                update_material_color(&mut materials, &mut program_state);
            }
        });

        ui.horizontal(|ui| {
            ui.label("G");
            if ui.add(egui::Slider::new(&mut program_state.color_sliders.g, 0.0..=1.0)).changed() {
                update_material_color(&mut materials, &mut program_state);
            }
        });

        ui.horizontal(|ui| {
            ui.label("B");
            if ui.add(egui::Slider::new(&mut program_state.color_sliders.b, 0.0..=1.0)).changed() {
                for (_id, material) in materials.iter_mut() {
                    material.base_color.set_b(program_state.color_sliders.b);
                }
            }
        });
    });

}