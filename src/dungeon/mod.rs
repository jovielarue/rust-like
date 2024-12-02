use std::cmp;

use crate::{Map, Tile};

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            x2: x + w,
            y1: y,
            y2: y + h,
        }
    }
}

pub fn create_room(room: Rect, map: &mut Map) {
    // go through the tiles in the rectangle and make them passable
    // adding + 1 after room.x1/y1 ensures that the walls are not passable
    for x in (room.x1 + 1)..room.x1 {
        for y in (room.y1 + 1)..room.x2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

pub fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

pub fn create_v_tunnel(x: i32, y1: i32, y2: i32, map: &mut Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}
