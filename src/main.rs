use bevy::{ prelude::*, sprite::MaterialMesh2dBundle, time::FixedTimestep };

const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 500.0;
const ARROW_SCALE: f32 = 0.01;
const ARROW_INTERVAL: usize = 20;
const FRAME_RATE: f64 = 0.01;
#[derive(Component, Debug, Default)]
pub struct Wire(pub f32);
#[derive(Component, Debug, Default)]
pub struct Arrow;
#[derive(Component, Debug, Default)]
pub struct Center(Vec3);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                window: WindowDescriptor {
                    width: WIDTH,
                    height: HEIGHT,
                    ..default()
                },
                ..default()
            })
        )
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FRAME_RATE))
                .with_system(interact)
                .with_system(move_wire)
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 0.0)),
            ..default()
        },
        Wire(-10.0),
        Center(Vec3::new(-200.0, 0.0, 0.0)),
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_translation(Vec3::new(250.0, 0.0, 0.0)),
            ..default()
        },
        Wire(1.0),
        Center(Vec3::new(200.0, 0.0, 0.0)),
    ));

    for x in ((-WIDTH as i32) / 2..(WIDTH as i32) / 2).step_by(ARROW_INTERVAL) {
        for y in ((-HEIGHT as i32) / 2..(HEIGHT as i32) / 2).step_by(ARROW_INTERVAL) {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("arrow.png"),
                    transform: Transform::from_xyz(x as f32, y as f32, 0.0).with_scale(
                        Vec3::new(ARROW_SCALE, ARROW_SCALE, ARROW_SCALE)
                    ),
                    ..default()
                },
                Arrow,
            ));
        }
    }
}

fn interact(
    mut arrows: Query<&mut Transform, (With<Arrow>, Without<Wire>)>,
    wires: Query<(&Transform, &Wire), (With<Wire>, Without<Arrow>)>
) {
    for mut arrow in arrows.iter_mut() {
        let mut wire_effect_global: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        for (wire_tran, wire) in wires.iter() {
            let mut wire_effect_local =
                (wire.0 *
                    (wire_tran.translation - arrow.translation).cross(Vec3::new(1.0, 1.0, 1.0))) /
                wire_tran.translation.distance_squared(arrow.translation).powf(1.5);
            wire_effect_local.z = 0.0;
            wire_effect_global += wire_effect_local;
        }
        let angle: f32 = wire_effect_global.y.atan2(wire_effect_global.x);
        arrow.rotation = Quat::from_rotation_z(angle);
    }
}

fn move_wire(mut wires: Query<(&mut Transform, &Center), With<Wire>>) {
    for (mut wire, center) in wires.iter_mut() {
        wire.rotate_around(center.0, Quat::from_rotation_z(0.01_f32));
    }
}