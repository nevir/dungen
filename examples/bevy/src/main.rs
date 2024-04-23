use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};

struct HexCell {
  q: i16,
  r: i16,
  s: i16,
  hue: f32,
}

struct HexGrid {
  cells: Vec<HexCell>,
}

#[derive(Component)]
struct Level {
  grid: HexGrid,
}

fn main() {
  App::new()
      .add_plugins(DefaultPlugins)
      .add_plugins(FramepacePlugin)
      .add_systems(Startup, setup)
      .add_systems(Update, render_grid)
      .run();
}

fn generate_grid() -> HexGrid {
  let radius = 20 as i16;
  let mut cells = Vec::new();
  for q in -radius..radius {
    for r in -radius..radius {
      let s = -q -r;
      if q.abs().max(r.abs()).max(s.abs()) > radius {
        continue;
      }

      cells.push(HexCell { q , r, s, hue: rand::random::<f32>() * 360.0 })
    }
  }

  return HexGrid { cells };
}

fn setup(
  mut commands: Commands,
  mut framepace_settings: ResMut<FramepaceSettings>,
) {
  // Because https://github.com/bevyengine/bevy/issues/10261
  framepace_settings.limiter = Limiter::from_framerate(5.0);

  commands.spawn(Camera2dBundle::default());
  commands.spawn(Level { grid: generate_grid() });
}

fn render_grid(
  query: Query<&Level, Changed<Level>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>
) {
  // TODO: How can we get this system to only run when the level changes? run_if()? Is that any better?
  if !query.get_single().is_ok() { return; }

  let level = query.get_single().unwrap();
  let cell_size = 10.0 as f32;

  for cell in &level.grid.cells {
    let x = cell_size * ((3.0 as f32).sqrt() * cell.q as f32 + (3.0 as f32).sqrt() / 2.0 * cell.r as f32);
    let y = cell_size * 3.0 / 2.0 * cell.r as f32;
    let color = Color::hsl(cell.hue, 0.5, 0.5);

    commands.spawn(MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(RegularPolygon::new(cell_size, 6))),
      material: materials.add(color),
      transform: Transform::from_xyz(x, y, 0.0),
      ..default()
    });
  }
}
