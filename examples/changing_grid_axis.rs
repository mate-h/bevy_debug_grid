use bevy::prelude::*;
use bevy_spectator::*;
use bevy_debug_grid::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpectatorPlugin)
        .add_plugin(DebugGridPlugin::without_floor_grid())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_demonstration_grid)
        .add_system(change_axis_color)
        .run();
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle::default(),
        Spectator,
    ));
}

#[derive(Component)]
struct ChangingAxis;

fn spawn_demonstration_grid(
    mut commands: Commands,
) {
    commands.spawn((
        Grid::default(),
        GridAxis::new_empty(),
        ChangingAxis,
        TransformBundle::default(),
        VisibilityBundle::default(),
    ));
}

fn change_axis_color(
    mut query: Query<&mut GridAxis, With<ChangingAxis>>,
    time: Res<Time>,
) {
    let elapsed = time.elapsed_seconds() * 0.25_f32;
    let selected_axis = (elapsed % 4.0_f32) as usize;
    let axis_color = Some(Color::Hsla {
        hue: (elapsed % 1.0_f32) * 360.0_f32,
        saturation: 0.5_f32,
        lightness: 0.5_f32,
        alpha: 1.0_f32,
    });
    for mut axis in query.iter_mut() {
        axis.x = None;
        axis.y = None;
        axis.z = None;
        match selected_axis {
            0 => axis.x = axis_color,
            1 => axis.y = axis_color,
            2 => axis.z = axis_color,
            _ => { },
        };
    }
}
