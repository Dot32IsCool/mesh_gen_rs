use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology, Indices};
use bevy::sprite::MaterialMesh2dBundle;

use noise::{NoiseFn, Perlin, Seedable};

const TAU: f32 = std::f32::consts::PI * 2.0;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::rgb(0.68, 0.97, 0.99)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // // Positions of the vertices
    // // See https://bevy-cheatbook.github.io/features/coords.html
    // mesh.insert_attribute(
    //     Mesh::ATTRIBUTE_POSITION,
    //     vec![[0.0, 0.0, 0.0], [100.0, 200.0, 0.0], [200.0, 0.0, 0.0]],
    // );

    // // In this example, normals and UVs don't matter,
    // // so we just use the same value for all of them
    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 3]);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);

    // // A triangle using vertices 0, 2, and 1.
    // // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
    // mesh.set_indices(Some(mesh::Indices::U32(vec![0, 2, 1])));

    // // commands.spawn(PbrBundle {
    // //     mesh: meshes.add(mesh),
    // //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    // //     ..default()
    // // });

    let mesh1 = build_circle(300.0, 256);

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh1).into(),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        material: materials.add(Color::rgb(108.0/255.0, 177.0/255.0, 5.0/255.0).into()),
        ..default()
    });

    let mesh2 = build_circle(295.0, 256);

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh2).into(),
        transform: Transform::from_xyz(0.0, 0.0, 2.0),
        material: materials.add(Color::rgb(0.61, 0.86, 0.26).into()),
        ..default()
    });

    let mesh3 = build_circle(275.0, 256);

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh3).into(),
        transform: Transform::from_xyz(0.0, 0.0, 3.0),
        material: materials.add(Color::rgb(0.53, 0.5, 0.43).into()),
        ..default()
    });

    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..default()
    // });

    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
    // commands.spawn(Camera2dBundle::default());
    // Spawn 2D camera with tonemapping dissabled
    commands.spawn(Camera2dBundle {
        tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::None,
        ..Default::default()
    });
}

pub fn build_circle(radius: f32, vertices: usize) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let perlin1 = Perlin::new(1);
    let perlin2 = Perlin::new(2);

    let n_vertices = vertices + 1;
    let n_triangles = vertices as u32;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(n_vertices);
    let mut indices: Vec<u32> = Vec::with_capacity((n_triangles as usize) * 3);

    for i in 0..vertices {
        let angle = (i as f32) / (vertices as f32) * TAU;

        let c = angle.cos();
        let s = angle.sin();

        let x = radius * c;
        let y = radius * s;

        let val = (perlin1.get([(x/radius*2.0).into(), (y/radius*2.0).into(), 10.0]) + perlin2.get([(x/radius*3.0).into(), (y/radius*3.0).into(), 300.0])*0.3)/1.3;

        let x = radius * c * (((val as f32) +0.5) + 5.0)/6.0;
        let y = radius * s * (((val as f32) +0.5) + 5.0)/6.0;

        let u = 0.5 * c + 0.5;
        let v = -0.5 * s + 0.5;

        positions.push([x, y, 0.]);
        normals.push([0., 0., 1.]);
        uvs.push([u, v]);
    }
    positions.push([0., 0., 0.]);
    normals.push([0., 0., 1.]);
    uvs.push([0.5, 0.5]);

    for i in 0..n_triangles {
        indices.push(i % n_triangles);
        indices.push((i + 1) % n_triangles);
        indices.push(n_triangles);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}