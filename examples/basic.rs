use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use bevy_dolly::{
    dolly_type::Rig, drivers::fpv::Fpv, helpers::cursor_grab::DollyCursorGrab, system::Dolly,
};
use bevy_lod_groups::{
    lod_group::{LodGroup, LodGroupPlugin},
    resolver::{LodResolver, LodResolverPlugin},
    CurrentLod, LodSettings,
};

struct LinearLodResolver;

impl LodResolver for LinearLodResolver {
    const RESOLUTION: f32 = 1.0;
    fn resolve_lod(distance_squared: f32) -> u8 {
        (distance_squared.sqrt() / 4.0).clamp(0.0, u8::MAX as f32) as u8
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut mesh_handles = Vec::new();
    let material = materials.add(Color::rgb(0.0, 0.0, 1.0).into());

    for i in 0..=6 {
        let mesh = meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 5.0,
                subdivisions: i,
            })
            .unwrap(),
        );
        mesh_handles.insert(0, mesh);
    }

    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, -10.0)),
        material,
        CurrentLod::default(),
        LodGroup::new(mesh_handles.into_boxed_slice()),
    ));

    commands.spawn((
        Camera3dBundle::default(),
        Rig::builder()
            .with(Fpv::from_position_target(Transform::IDENTITY))
            .build(),
    ));
    commands.init_resource::<AmbientLight>()
}

fn update_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut rig_query: Query<&mut Rig>,
) {
    let time_delta_seconds: f32 = time.delta_seconds();
    let sensitivity = Vec2::splat(1.0);

    let mut move_vec = Vec3::ZERO;

    if keys.pressed(KeyCode::W) {
        move_vec.z -= 1.0;
    }
    if keys.pressed(KeyCode::S) {
        move_vec.z += 1.0;
    }
    if keys.pressed(KeyCode::A) {
        move_vec.x -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        move_vec.x += 1.0;
    }

    if keys.pressed(KeyCode::E) || keys.pressed(KeyCode::Space) {
        move_vec.y += 1.0;
    }
    if keys.pressed(KeyCode::Q) {
        move_vec.y -= 1.0;
    }

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }
    delta.x *= sensitivity.x;
    delta.y *= sensitivity.y;

    let mut rig = rig_query.single_mut();

    if let Ok(window) = windows.get_single() {
        if !window.cursor.visible {
            rig.driver_mut::<Fpv>()
                .update_pos_rot(move_vec, delta, false, 1.0, time_delta_seconds);
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DollyCursorGrab,
            LodResolverPlugin::<Camera3d, LinearLodResolver>::default(),
            LodGroupPlugin::<Handle<Mesh>>::default(),
        ))
        .insert_resource(LodSettings { bias: -4 })
        .add_systems(Startup, setup)
        .add_systems(Update, (Dolly::<Camera3d>::update_active, update_camera))
        .run();
}
