use tcod::colors::*;
use tcod::console::*;
mod dungeon;
mod map;
use map::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

struct Tcod {
    root: Root,
    con: Offscreen,
}

// Object is a generic structure that can represent anything on screen with a character
#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    // Instantiate a new object
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    // Move the object by dx or dy
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        // if a tile is not blocked at current position + dx/dy, move by dx/dy
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    // render the object character on screen
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

// Handles keypresses and moves the player according to those keypresses
fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            let full_screen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!full_screen); // sets fullscreen on alt+enter
        }
        Key { code: Escape, .. } => return true, // exits game

        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),
        _ => {}
    }

    false
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    // Initializes root console
    let root: Root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    // creates a new offscreen console for rendering on top of the root console
    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    let mut tcod = Tcod { root, con };

    // creates a player and npc object and stores them in an objects array
    let player = Object::new(25, 23, '@', WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', YELLOW);
    let mut objects = [player, npc];

    // creates new game using functionality from the map module
    let game = Game { map: make_map() };

    // the game loop
    while !tcod.root.window_closed() {
        tcod.con.clear();

        // function from the map module
        render_all(&mut tcod, &game, &objects);

        tcod.root.flush();

        let player = &mut objects[0];
        // exit returns true on pressing the escape key
        let exit = handle_keys(&mut tcod, &game, player);
        if exit {
            break;
        };
    }
}
