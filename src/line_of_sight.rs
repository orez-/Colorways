use std::collections::HashSet;
use geo_visibility::Visibility as _;
use crate::room::Room;

const TILE_SIZE: f64 = 16.;

pub struct Visibility {
    pub polygon_pts: Vec<[f64; 2]>,
    pub tiles: HashSet<(i32, i32)>,
}

impl Visibility {
    fn new() -> Self {
        Self { polygon_pts: Vec::new(), tiles: HashSet::new() }
    }
}

pub fn line_of_sight(viewer_x: i32, viewer_y: i32, width: usize, height: usize, walls_polygon: &geo::MultiPolygon<f64>) -> Visibility {
    let viewer = geo::Point::new(
        (viewer_x as f64 + 0.5) * TILE_SIZE,
        (viewer_y as f64 + 0.5) * TILE_SIZE,
    );
    let polygon = viewer.visibility(walls_polygon);
    let mut vis = Visibility::new();
    // just give me the floats. for gods sake please just give me the floats.
    // i think i hate this library
    let (line, _) = polygon.into_inner();
    vis.polygon_pts = line.into_points().into_iter().map(|p| [p.x(), p.y()]).collect();
    vis
}
