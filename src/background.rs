use bevy::prelude::Handle;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::prelude::{default, ColorMaterial, Commands, Mesh, Rectangle, ResMut, Transform};

pub fn add_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Build a default quad mesh
    let mut mesh = Mesh::from(Rectangle::default());
    // Build vertex colors for the quad. One entry per vertex (the corners of the quad)
    let vertex_colors: Vec<[f32; 4]> = vec![
        [1.0, 0.2, 0.3, 1.0], // Top-right
        [1.0, 0.7, 0.8, 1.0], // Top left
        [1.0, 0.2, 0.4, 1.0], // Bottom-left
        [1.0, 0.6, 0.7, 1.0], //Bottom-right
    ];
    // Insert the vertex colors as an attribute
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);

    let mesh_handle: Handle<Mesh> = meshes.add(mesh).into();

    // Spawn the quad with vertex colors
    commands.spawn(
(
        mesh_handle.clone(),
        Transform::from_translation(Vec3::new(0.0, 0.0, -0.1))
            .with_scale(Vec3::splat(2048.0)),
        materials.add(ColorMaterial::default())
        )
    );
}
