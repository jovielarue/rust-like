use tcod::{console::blit, BackgroundFlag, Color, Console};

use crate::{Object, Tcod};

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

// custom colors for map tiles
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 15,
};

// A tile can be blocked (e.g. player cannot move through it) as well as block the players'
// sightline
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
}

impl Tile {
    // create empty map tile
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    // create a map tile that players cannot walk/see through
    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

// Map is a 2-dimensional vector containing tiles
type Map = Vec<Vec<Tile>>;

pub struct Game {
    pub map: Map,
}

// creates a map full of unblocked tiles and two walls
pub fn make_map() -> Map {
    // fills the vectors with MAP_WIDTH/MAP_HEIGHT number of tiles
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // place two walls in the map
    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();

    map
}

// renders all objects on screen
pub fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object]) {
    for object in objects {
        object.draw(&mut tcod.con);
    }

    // places walls or tiles based on whether or not the map tile blocks sight
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    // blit is a tcod function that places the offscreen console (con) on top of the root console.
    blit(
        &tcod.con,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}
