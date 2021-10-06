use std::collections::HashMap;

mod board;
mod move_req;
mod piece_directions;
mod piece_enums;
mod piece_moves;

fn main() {
    let something = piece_moves::generate_moves(38);
    println!("here: {:?}", something);
    #[derive(Debug)]
    struct Nesto {
        ime: String,
    }

    /*
        impl Nesto {
            pub fn update_ime(&mut self, mut novo: String) {
                self.ime = novo;
            }
        }

        let mut sve: HashMap<i32, Nesto> = HashMap::new();
        sve.insert(
            33,
            Nesto {
                ime: String::from("Uros"),
            },
        );
        let uros = sve.get(&33);
        match uros {
            Some(i) => {}
            None => (),
        }
    */
}
