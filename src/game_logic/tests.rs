// trying to do learn unit testing 
// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html



// use std::default;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
 
    //use super::*;
    //#[cfg(test)]   
    //use itertools::Either;
    #[cfg(test)]   
    use super::structs::{Direction,Row,GameFrame,GameState};
    #[cfg(test)]   
    use super::logic::{create_range,move_squares,check_valid};
    //Game_structs section -- utility function testing
    #[test]
    fn test_rangecreate() -> Result<(), String> {
        let size: usize = 4;

        let zz:itertools::Either<std::ops::Range<usize>, std::ops::Range<usize>> = itertools::Either::Left(0..size);
        
        let iter4 = match create_range(false,size) {
            Ok(res) => res,
            Err(err) => panic!("Problem making range, {:?}", err)
        };
        itertools::assert_equal(zz, iter4);
        Ok(())
    }
    #[test]
    fn test_rangecreate_rev() -> Result<(), String> {
        let size: usize = 4;

        let zz:itertools::Either<std::ops::Range<usize>, std::iter::Rev<std::ops::Range<usize>>> = itertools::Either::Right((0..size).rev());
        
        let iter4 = match create_range(true,size) {
            Ok(res) => res,
            Err(err) => panic!("Problem making reversed range, {:?}", err)
        };
        itertools::assert_equal(zz, iter4);
        Ok(())
    }
    // Movement testing / validation
    #[test]
    fn test_move_up() -> Result<(), String> {
        let dir: Direction = Direction::Up;
        let demo_before: GameFrame = GameFrame(
            [Row([4,2,0,0]),
            Row([2,0,0,0]),
            Row([0,0,2,0]),
            Row([0,0,0,4])]);
        let demo_after: GameFrame = GameFrame(
            [Row([4,2,2,4]),
            Row([2,0,0,0]),
            Row([0,0,0,0]),
            Row([0,0,0,0])
            ]);
        let demo_state: GameState = GameState {current: demo_before, ..Default::default()};
        let test_after = move_squares(dir, demo_state);
        assert_eq!(test_after.current, demo_after);
        Ok(())
    }
    #[test]
    fn test_move_down() -> Result<(), String> {
        let dir: Direction = Direction::Down;
        let demo_before: GameFrame = GameFrame(
            [Row([4,2,0,0]),
            Row([2,0,0,0]),
            Row([0,0,2,0]),
            Row([0,0,0,4])]);
        let demo_after: GameFrame = GameFrame(
            [Row([0,0,0,0]),
            Row([0,0,0,0]),
            Row([4,0,0,0]),
            Row([2,2,2,4])
            ]);
        let demo_state: GameState = GameState {current: demo_before, ..Default::default()};
        let test_after = move_squares(dir, demo_state);
        assert_eq!(test_after.current, demo_after);
        Ok(())
    }
    #[test]
    fn test_move_left() -> Result<(), String> {
        let dir: Direction = Direction::Left;
        let demo_before: GameFrame = GameFrame(
            [Row([4,2,0,0]),
            Row([2,0,0,0]),
            Row([0,0,2,0]),
            Row([0,0,0,4])]);
        let demo_after: GameFrame = GameFrame(
            [Row([4,2,0,0]),
            Row([2,0,0,0]),
            Row([2,0,0,0]),
            Row([4,0,0,0])
            ]);
        let demo_state: GameState = GameState {current: demo_before, ..Default::default()};
        let test_after = move_squares(dir, demo_state);
        assert_eq!(test_after.current, demo_after);
        Ok(())
    }
    #[test]
    fn test_move_right() -> Result<(), String> {
        let dir: Direction = Direction::Right;
        let demo_before: GameFrame = GameFrame(
            [Row([4,2,0,0]),
            Row([2,0,0,0]),
            Row([0,0,2,0]),
            Row([0,0,0,4])]);
        let demo_after: GameFrame = GameFrame(
            [Row([0,0,4,2]),
            Row([0,0,0,2]),
            Row([0,0,0,2]),
            Row([0,0,0,4])
            ]);
        let demo_state: GameState = GameState {current: demo_before, ..Default::default()};
        let test_after = move_squares(dir, demo_state);
        assert_eq!(test_after.current, demo_after);
        Ok(())
    }
    #[test]
    fn test_valid_move_up() -> Result<(), String> {
        let dir: Direction = Direction::Up;
        let demo_before: GameFrame = GameFrame(
            [Row([4,2,0,0]),
            Row([2,0,0,0]),
            Row([0,0,2,0]),
            Row([0,0,0,4])]);
        let demo_state: GameState = GameState {current: demo_before, ..Default::default()};
        let validity = check_valid(dir, &demo_state);
        assert_eq!(validity, true);
        Ok(())
    }
    #[test]
    fn test_invalid_move_up() -> Result<(), String> {
        let dir: Direction = Direction::Up;
        let demo_before: GameFrame = GameFrame(
            [Row([4,2,2,16]),
            Row([2,0,4,8]),
            Row([0,0,2,2]),
            Row([0,0,4,4])]);
        let demo_state: GameState = GameState {current: demo_before, ..Default::default()};
        let validity = check_valid(dir, &demo_state);
        assert_eq!(validity, false);
        Ok(())
    }
    #[test]
    fn test_valid_move_down() -> Result<(), String> {
        let dir: Direction = Direction::Down;
        let demo_before: GameFrame = GameFrame(
            [Row([0,2,0,0]),
            Row([0,0,0,0]),
            Row([0,0,2,0]),
            Row([0,0,0,4])]);
        let demo_state: GameState = GameState {current: demo_before, ..Default::default()};
        let validity = check_valid(dir, &demo_state);
        assert_eq!(validity, true);
        Ok(())
    }
    #[test]
    fn test_invalid_move_down() -> Result<(), String> {
        let dir: Direction = Direction::Down;
        let demo_before: GameFrame = GameFrame(
            [Row([0,0,2,16]),
            Row([0,0,4,8]),
            Row([0,0,2,2]),
            Row([2,2,4,4])]);
        let demo_state: GameState = GameState {current: demo_before, ..Default::default()};
        let validity = check_valid(dir, &demo_state);
        assert_eq!(validity, false);
        Ok(())
    }