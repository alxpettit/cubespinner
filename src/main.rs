use std::f32::consts::PI;

use bevy::{prelude::*, reflect::erased_serde::__private::serde::__private::de};
use bevy_egui::{egui, EguiContext, EguiPlugin};


use bevy_infinite_grid::{InfiniteGridPlugin, InfiniteGridBundle, InfiniteGrid, GridShadowCamera};
use bevy_mod_picking::{DefaultPickingPlugins, PickingCameraBundle, PickableBundle, PickingEvent};
use bevy_transform_gizmo::{TransformGizmoPlugin, GizmoTransformable, GizmoPickSource};
use rand::prelude::*;

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
    color_sliders: ColorF32,
    expose_gay: bool
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(InfiniteGridPlugin)
        .add_plugin(TransformGizmoPlugin::default())
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_system(ui_example)
        .add_system(rotate)
        .add_system_to_stage(CoreStage::PostUpdate, pick_event)
        .run();
}

fn pick_event(mut events: EventReader<PickingEvent>, mut program_state: ResMut<ProgramState>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("Select event: {:?}", e),
            PickingEvent::Hover(e) => info!("Hover event: {:?}", e),
            PickingEvent::Clicked(e) => {
                info!("Click event: {:?}", e);
                program_state.expose_gay = true;
            },
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) { 
    commands.insert_resource(ProgramState::default());
    let infinite_grid_bundle = InfiniteGridBundle {
        grid: InfiniteGrid {
            ..default()
        },
        ..default()
    };
    commands.spawn(infinite_grid_bundle);

    let debug_material = materials.add(StandardMaterial {
        //base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let shapes = [
        meshes.add(shape::Cube::default().into())
    ];
    for (_i, shape) in shapes.into_iter().enumerate() {
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
        commands.spawn((pbr, Shape, PickableBundle::default(), GizmoTransformable));
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

    commands.spawn((camera_3d_bundle,
        PickingCameraBundle::default(),
        GizmoPickSource::default()
        ))
        .insert(GridShadowCamera);
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

fn ui_example(mut egui_context: ResMut<EguiContext>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut program_state: ResMut<ProgramState>) {

    let update_material_color =  |materials: &mut ResMut<Assets<StandardMaterial>>, program_state: &mut ResMut<ProgramState>| {
        for (_id, material) in materials.iter_mut() {
            material.base_color = Color::rgb_linear(program_state.color_sliders.r,
            program_state.color_sliders.g,
            program_state.color_sliders.b);
        }
    };

    if program_state.expose_gay {
        egui::Window::new("UWU").show(egui_context.ctx_mut(), |ui| {
            ui.label("GAAYYYYYYYY");
            if ui.button("yes thsank u").clicked() {
                program_state.expose_gay = false;
            }
        });
    }

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