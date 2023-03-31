mod game_structs;
use std::{thread, time};

use console::Term;

fn main() {

    let mut x: game_structs::GameState = 
    game_structs::new_game(game_structs::init_state());
    println!("{}", x);
    let upres: bool =game_structs::check_valid(game_structs::Direction::Up, &x);
    let downres: bool =game_structs::check_valid(game_structs::Direction::Down, &x);
    let leftres: bool =game_structs::check_valid(game_structs::Direction::Left, &x);
    let rightres: bool =game_structs::check_valid(game_structs::Direction::Right, &x);

    println!("Is Up a valid move? {}\nIs Down a valid move? {}\nIs Left a valid move? {}\nIs Right a valid move? {}\n", 
    upres,downres,leftres,rightres);
/* 
    x = game_structs::process_move(game_structs::Direction::Up, x);
    println!("{}", x);
    x = game_structs::process_move(game_structs::Direction::Down, x);
    println!("{}", x);
    x = game_structs::process_move(game_structs::Direction::Left, x);
    println!("{}", x);
    x = game_structs::process_move(game_structs::Direction::Right, x);
    */
    let term = Term::stdout();


    loop {
        let keydown_res = term.read_key();
        let keydown = match keydown_res {
            Ok(key) => key,
            Err(error) => panic!("Problem reading keydown: {:?}", error),
        };
        match keydown {
            console::Key::ArrowUp => {
                x = game_structs::process_move(game_structs::Direction::Up, x);
            },
            console::Key::ArrowDown => {
                x = game_structs::process_move(game_structs::Direction::Down, x);
            },
            console::Key::ArrowLeft => {
                x = game_structs::process_move(game_structs::Direction::Left, x);
            },
            console::Key::ArrowRight => {
                x = game_structs::process_move(game_structs::Direction::Right, x);
            },
            _ => {

            }
        }
        term.clear_screen();
        println!("{}", x);
        thread::sleep(time::Duration::from_millis(100));
    }
    
    
    //thread::sleep(time::Duration::from_secs(2));
    // println!("Previous Frame:\n{}", x);
}
