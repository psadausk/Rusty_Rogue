use entities::movement::input::Input;
use entities::object::Object;
use map::map::Map;
use tcod::console::{self, Offscreen, Root};
use tcod::input::Key;
use tcod::input::KeyCode::*;

//Also handles menus and stuff?
// pub struct PlayerMovement {
//     root: Root,
// }

// impl PlayerMovement {
//     pub fn new(root: &Root) -> PlayerMovement {
//         PlayerMovement { root }
//     }
// }

// impl Input for PlayerMovement {
//     fn update(&mut self, player: &mut Object, map: &Map) {
//         let key = self.root.wait_for_keypress(true);
//         match key {
//             Key {
//                 code: Enter,
//                 alt: true,
//                 ..
//             } => {
//                 // Alt+Enter: toggle fullscreen
//                 let fullscreen = self.root.is_fullscreen();
//                 self.root.set_fullscreen(!fullscreen);
//             }
//             //Key { code: Escape, .. } => std::process::exit,  // exit game

//             // movement keys
//             Key { code: Spacebar, .. } => {}
//             Key { code: Up, .. } => player.move_by(0, -1, &map),
//             Key { code: Down, .. } => player.move_by(0, 1, &map),
//             Key { code: Left, .. } => player.move_by(-1, 0, &map),
//             Key { code: Right, .. } => player.move_by(1, 0, &map),

//             _ => {}
//         }
//     }
// }
