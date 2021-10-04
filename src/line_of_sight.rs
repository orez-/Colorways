use geo::polygon;
use geo::prelude::Contains;
use geo_visibility::Visibility as _;

const TILE_SIZE: f64 = 16.;

pub struct Visibility {
    pub polygon_pts: Vec<[f64; 2]>,
    pub tiles: Vec<usize>,
}

pub fn line_of_sight(viewer_x: i32, viewer_y: i32, width: usize, height: usize, walls_polygon: &geo::MultiPolygon<f64>) -> Visibility {
    let viewer = geo::Point::new(
        (viewer_x as f64 + 0.5) * TILE_SIZE,
        (viewer_y as f64 + 0.5) * TILE_SIZE,
    );
    let vis_poly = viewer.visibility(walls_polygon);

    let mut tiles = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let xf = x as f64 * TILE_SIZE;
            let yf = y as f64 * TILE_SIZE;
            let tile_poly = polygon![
                geo::Coordinate { x: xf, y: yf },
                geo::Coordinate { x: xf + TILE_SIZE, y: yf },
                geo::Coordinate { x: xf + TILE_SIZE , y: yf + TILE_SIZE },
                geo::Coordinate { x: xf, y: yf + TILE_SIZE },
            ];
            if vis_poly.contains(&tile_poly) {
                let idx = x + y * width;
                tiles.push(idx);
            }
        }
    }

    // just give me the floats. for gods sake please just give me the floats.
    let (line, _) = vis_poly.into_inner();
    let polygon_pts = line.into_points().into_iter().map(|p| [p.x(), p.y()]).collect();
    Visibility { polygon_pts, tiles }
}
