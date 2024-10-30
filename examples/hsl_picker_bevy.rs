//! Shows how to render to a texture. Useful for mirrors, UI, or exporting images.

use std::{
    any::{Any, TypeId},
    f32::consts::PI,
};

use bevy::{
    color::palettes::css::{BLUE, RED},
    ecs::bundle::DynamicBundle,
    prelude::*,
    reflect::TypeRegistry,
    render::{
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        view::RenderLayers,
    },
    utils::HashMap,
    window::WindowResolution,
};
use bevy_ui_inspector::{
    element::{spawn_element_hierarchy, ComponentArgs, Element, ElementChildren},
    val::ValExt,
    UiInspectorPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920., 1080.),
                ..default()
            }),
            ..default()
        }))
        .register_type::<MyComponentA>()
        .register_type::<MyComponentB>()
        .add_plugins(UiInspectorPlugin)
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (cube_rotator_system, rotator_system))
        .add_systems(Update, (teset))
        .run();
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    hue: u32,
}
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "./hsl_picker.wgsl".into()
    }
}
// Marks the first pass cube (rendered to a texture.)
#[derive(Component)]
struct FirstPassCube;

// Marks the main pass cube, to which the texture is applied.
#[derive(Component)]
struct MainPassCube;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct MyComponentA {
    value: i32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct MyComponentB {
    text: String,
}

fn box_component(args: impl Into<ComponentArgs>) -> Element {
    Element {
        node: NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Stretch,
                align_items: AlignItems::Stretch,
                ..default()
            },
            background_color: BLUE.into(),
            ..default()
        },
        components: HashMap::new(),
        children: args.into().children,
    }
}

fn row_component() -> Element {
    Element {
        node: NodeBundle {
            style: Style {
                height: Val::Px(100.0),
                ..default()
            },
            background_color: RED.into(),
            ..default()
        },
        components: HashMap::new(),
        children: ElementChildren::Elements(vec![]),
    }
}
fn teset(query: Query<&MyComponentA>, queryw: Query<&MyComponentB>) {
    // dbg!(query.iter().len(), queryw.iter().len());
}
fn setup(world: &mut World) {
    let my_struct = box_component([
        row_component().with_text("Poo", TextStyle::default()),
        row_component().add_component(MyComponentB {
            text: "hoi".to_string(),
        }),
    ])
    .add_component(MyComponentA { value: 10 })
    .add_children([row_component().with_style(|style| {
        style.margin = UiRect::left(10.0.px());
    })]);
    spawn_element_hierarchy(my_struct, world, None);

    //     let window = windows.get_single().unwrap();
    //     let aspect_ratio = window.width() / window.height();

    //     // Define the camera's field of view (in radians)
    //     let fov_y = f32::to_radians(60.0); // Default is 60 degrees

    //     // Set the distance from the camera to the quad
    //     let z = -1.0;

    //     // Calculate the quad's height and width to cover the full screen
    //     let height = 2.0 * (-z) * (fov_y / 2.0).tan();
    //     let width = height * aspect_ratio;
    //     dbg!(width, height);
    //     let size = Extent3d {
    //         width: window.width() as u32,
    //         height: window.height() as u32,
    //         ..default()
    //     };

    //     // This is the texture that will be rendered to.
    //     let mut image = Image {
    //         texture_descriptor: TextureDescriptor {
    //             label: None,
    //             size,
    //             dimension: TextureDimension::D2,
    //             format: TextureFormat::Bgra8UnormSrgb,
    //             mip_level_count: 1,
    //             sample_count: 1,
    //             usage: TextureUsages::TEXTURE_BINDING
    //                 | TextureUsages::COPY_DST
    //                 | TextureUsages::RENDER_ATTACHMENT,
    //             view_formats: &[],
    //         },
    //         ..default()
    //     };

    //     // fill image.data with zeroes
    //     image.resize(size);

    //     let image_handle = images.add(image);

    //     let first_pass_layer = RenderLayers::layer(1);
    //     // Spawn the camera
    //     let camera_entity = commands
    //         .spawn((
    //             Camera3dBundle {
    //                 transform: Transform::default(),
    //                 projection: PerspectiveProjection {
    //                     aspect_ratio: 1.0,
    //                     fov: f32::to_radians(60.0),
    //                     near: 0.1,
    //                     far: 1000.0,
    //                 }
    //                 .into(),
    //                 camera: Camera {
    //                     // render before the "main pass" camera
    //                     order: -1,
    //                     target: image_handle.clone().into(),
    //                     clear_color: Color::WHITE.into(),
    //                     ..default()
    //                 },
    //                 ..Default::default()
    //             },
    //             first_pass_layer.clone(),
    //         ))
    //         .id();

    //     // Get the primary window to determine the aspect ratio

    //     // Create a quad mesh
    //     let quad_mesh = meshes.add(Mesh::from(Rectangle::default()));

    //     // Create a unlit material (optional: customize as needed)
    //     let quad_material = materials.add(CustomMaterial { hue: 200 });

    //     // Spawn the quad with the calculated scale and position
    //     commands.spawn((
    //         MaterialMeshBundle {
    //             mesh: quad_mesh,
    //             material: quad_material,
    //             transform: Transform {
    //                 translation: Vec3::new(0.0, 0.0, z),
    //                 scale: Vec3::new(width, height, 1.0),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         },
    //         first_pass_layer.clone(),
    //     ));

    //     // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.

    //     // The cube that will be rendered to the texture.
    //     // commands.spawn((
    //     //     PbrBundle {
    //     //         mesh: cube_handle,
    //     //         material: cube_material_handle,
    //     //         transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
    //     //         ..default()
    //     //     },
    //     //     FirstPassCube,
    //     //     first_pass_layer.clone(),
    //     // ));

    //     // Light
    //     // NOTE: we add the light to both layers so it affects both the rendered-to-texture cube, and the cube on which we display the texture
    //     // Setting the layer to RenderLayers::layer(0) would cause the main view to be lit, but the rendered-to-texture cube to be unlit.
    //     // Setting the layer to RenderLayers::layer(1) would cause the rendered-to-texture cube to be lit, but the main view to be unlit.
    //     commands.spawn((
    //         PointLightBundle {
    //             transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
    //             ..default()
    //         },
    //         RenderLayers::layer(0).with(1),
    //     ));

    //     let cube_handle = meshes.add(Rectangle::new(2.0, 2.0));

    //     // This material has the texture that has been rendered.
    //     let material_handle = st_materials.add(StandardMaterial {
    //         base_color_texture: Some(image_handle),
    //         reflectance: 0.02,
    //         unlit: true,
    //         ..default()
    //     });

    //     // Main pass cube, with material containing the rendered first pass texture.
    //     commands.spawn((
    //         PbrBundle {
    //             mesh: cube_handle,
    //             material: material_handle,
    //             transform: Transform::from_xyz(0.0, 0.0, 1.5), //.with_scale(Vec3::new(1.0,5.0,1.0)) ,
    //             ..default()
    //         },
    //         MainPassCube,
    //     ));

    // The main pass camera.
    world.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassCube>>) {}

/// Rotates the outer cube (main pass)
fn cube_rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<MainPassCube>>) {}

// use bevy::prelude::*;
// use bevy::render::{
//     camera::RenderTarget,
//     render_resource::{
//         AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
//         TextureUsages,
//     },
// };
// use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle};

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugins(Material2dPlugin::<CustomMaterial>::default())
//         .add_systems (Startup, setup)
//         .run();
// }

// #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// pub struct CustomMaterial {}

// impl Material2d for CustomMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "./hsl_picker.wgsl".into()
//     }
// }

// fn setup(
//     mut commands: Commands,
//     mut images: ResMut<Assets<Image>>,
//     mut custom_materials: ResMut<Assets<CustomMaterial>>,
//     mut meshes: ResMut<Assets<Mesh>>,
// ) {
//     // Create the render target texture (256x256 pixels)
//     let size = Extent3d {
//         width: 256,
//         height: 256,
//         depth_or_array_layers: 1,
//     };
//     let image = Image {
//         texture_descriptor: TextureDescriptor {
//             label: Some("Render Target Texture"),
//             size,
//             dimension: TextureDimension::D2,
//             format: TextureFormat::Rgba8UnormSrgb,
//             usage: TextureUsages::TEXTURE_BINDING
//                 | TextureUsages::COPY_DST
//                 | TextureUsages::RENDER_ATTACHMENT,
//             mip_level_count: 1,
//             sample_count: 1,
//             view_formats: &[],
//         },
//         data: vec![0; (size.width * size.height * 4) as usize],
//         ..Default::default()
//     };
//     let image_handle = images.add(image);

//     // Set up an off-screen camera that renders to the texture
//     commands.spawn(Camera2dBundle {
//         camera: Camera {
//             order: -1, // Render before the main camera
//             target: RenderTarget::Image(image_handle.clone()),
//             ..Default::default()
//         },
//         ..Default::default()
//     });

//     // Spawn a quad that uses the custom fragment shader
//     commands.spawn(MaterialMesh2dBundle {
//         mesh: Mesh2dHandle(meshes.add(Mesh::from(Rectangle::new(256.0, 256.0)))),
//         material: custom_materials.add(CustomMaterial {}),
//         transform: Transform::from_scale(Vec3::splat(2.0)), // Scale to cover the view
//         ..Default::default()
//     });

//     // Spawn a sprite to display the rendered texture
//     commands.spawn(SpriteBundle {
//         texture: image_handle.clone(),
//         transform: Transform::from_xyz(0.0, 0.0, 0.0), // Position the sprite
//         sprite: Sprite {
//             custom_size: Some(Vec2::new(256.0, 256.0)), // Size of the sprite
//             ..Default::default()
//         },
//         ..Default::default()
//     });

//     // Add a main camera to view the sprite
//     commands.spawn(Camera2dBundle::default());
// }
