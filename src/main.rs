mod game_logic;
//mod tests;
use std::{thread, time};
use game_logic::logic::*;
use game_logic::structs::*;
use console::Term;

fn main() {

    let mut x: GameState = 
    new_game(init_state());
    println!("{}", x);
    let upres: bool =check_valid(Direction::Up, &x);
    let downres: bool =check_valid(Direction::Down, &x);
    let leftres: bool =check_valid(Direction::Left, &x);
    let rightres: bool =check_valid(Direction::Right, &x);

    println!("Is Up a valid move? {}\nIs Down a valid move? {}\nIs Left a valid move? {}\nIs Right a valid move? {}\n", 
    upres,downres,leftres,rightres);
/* 
    x = game_logic::process_move(game_logic::Direction::Up, x);
    println!("{}", x);
    x = game_logic::process_move(game_logic::Direction::Down, x);
    println!("{}", x);
    x = game_logic::process_move(game_logic::Direction::Left, x);
    println!("{}", x);
    x = game_logic::process_move(game_logic::Direction::Right, x);
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
                x = process_move(Direction::Up, x);
            },
            console::Key::ArrowDown => {
                x = process_move(Direction::Down, x);
            },
            console::Key::ArrowLeft => {
                x = process_move(Direction::Left, x);
            },
            console::Key::ArrowRight => {
                x = process_move(Direction::Right, x);
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
