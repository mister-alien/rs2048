// import fmt to make available
    use std::fmt;
    use rand::Rng;
    use itertools::Either;
    //use to make selectable iterators.
    // All possible game moves :)
    #[derive(Copy, Clone, PartialEq)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right
    }
 
    // Individual coordinate structure.. maybe for later?
    struct Coordinate {x: usize, y: usize }
    impl Default for Coordinate {
        fn default() -> Coordinate {
            Coordinate { x: 0, y: 0 }
        }
    }
    #[derive(Debug)]
    pub struct Row(pub [u32; 4]);
    impl Default for Row {
        fn default() -> Row {
            Row([0,0,0,0])
        }
    }
    impl PartialEq for Row {
        fn eq(&self, other: &Self) -> bool {
            (self.0[0] == other.0[0]) && (self.0[1] ==other.0[1]) && 
            (self.0[2] == other.0[2]) && (self.0[3] == other.0[3])
        }
    }
    impl fmt::Display for Row {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"[{},{},{},{}]", self.0[0],self.0[1],self.0[2],self.0[3])
        }
    }
    #[derive(Debug)]
    pub struct GameFrame(pub [Row; 4]);
    impl Default for GameFrame {
        fn default() -> GameFrame {
            GameFrame([Row(Default::default()),
                    Row(Default::default()),
                    Row(Default::default()),
                    Row(Default::default())])
        }
    }
    impl PartialEq for GameFrame {
        fn eq(&self, other: &Self) -> bool {
            (self.0[0] == other.0[0]) && (self.0[1] ==other.0[1]) && 
            (self.0[2] == other.0[2]) && (self.0[3] == other.0[3])
        }
    }
    impl fmt::Display for GameFrame {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"{}\n{}\n{}\n{}", 
                self.0[0],
                self.0[1],
                self.0[2],
                self.0[3])
        }
    }
    pub struct GameState {
        pub current: GameFrame,
        pub prev: GameFrame,
        pub moves: u32,
    }
    impl Default for GameState {
        fn default() -> GameState {
            GameState { current: init_state(), 
                prev: GameFrame(Default::default()), 
                moves: 0 }
        }
    }
    impl fmt::Display for GameState {

        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"Move: {}\n{}", self.moves, 
                self.current)
        }
    }

    // CREATES range UNTIL size (range of size "size" )
    pub fn create_range(
        rev: bool,  size: usize) -> Result<itertools::Either<impl Iterator<Item = usize>, impl Iterator<Item = usize>>, String> {
         if !rev {
            Ok(itertools::Either::Left(0..size))
        } else {
            Ok(itertools::Either::Right((0..size).rev()))
        }
    }
    pub fn merge_adjacent(dir:Direction, state: GameState) -> GameState {
        let mut valid_merge: bool;
        let mut merge_val: u32;
        let mut cur_state: GameState = state;
        let mut check_value: u32;
        let mut merge_col: usize;
        let mut merge_row: usize;

        let mut c: usize;
        let mut r: usize;

        let rev: bool = dir == Direction::Right || dir == Direction::Down;
        let outer_loop_range = match create_range(false, 4) {
            Ok(range) => range,
            Err(error) => panic!("Problem Creating Range, {:?}", error),
        };
        for outer in outer_loop_range {
            
            valid_merge = false;
            merge_val = 0;
            merge_col = 0;
            merge_row = 0;
            let inner_loop_range = match create_range(rev, 4) {
                Ok(range) => range,
                Err(error) => panic!("Problem Creating Range, {:?}", error),
            };
            for inner in inner_loop_range {
                if dir == Direction::Right || dir == Direction::Left {
                    r = outer;
                    c = inner;
                }else{
                    r = inner;
                    c = outer;
                }
                check_value = cur_state.current.0[r].0[c];
                if check_value == 0 {
                    valid_merge = false;
                } else if check_value > 0 && !valid_merge {
                    merge_col = c;
                    merge_row = r;
                    merge_val = check_value;
                    valid_merge = true;
                } else if check_value >0 && valid_merge {
                    if check_value != merge_val {
                        merge_val = check_value;
                        merge_row = r;
                        merge_col = c;
                    } else {
                        cur_state.current = merge_squares(Coordinate{x:merge_col,y:merge_row}, Coordinate{x:c,y:r}, merge_val*2, cur_state.current);
                        valid_merge = false;
                    }
                }

            }
        }
        cur_state
    }
    pub fn move_squares(dir: Direction, state: GameState) -> GameState {
        let mut open_spot_flag: bool;
        let mut cur_state = state;
        let mut open_col: usize = 0;
        let mut open_row: usize = 0;
        let mut c: usize;
        let mut r: usize;
        let rev: bool = dir == Direction::Right || dir == Direction::Down;

        // we're introducing result enums because (shrug)
        let outer_loop_range = match create_range(false, 4) {
            Ok(range) => range,
            Err(error) => panic!("Problem Creating Range, {:?}", error),
        };


        for outer in outer_loop_range {
            open_spot_flag = false;
            let inner_loop_range = match create_range(rev, 4) {
                Ok(range) => range,
                Err(error) => panic!("Problem Creating Range, {:?}", error),
            };
            for inner in inner_loop_range {
                
                if dir == Direction::Right || dir == Direction::Left {
                    r = outer;
                    c = inner;
                }else{
                    r = inner;
                    c = outer;
                }
                if cur_state.current.0[r].0[c] == 0 {
                    if !open_spot_flag {
                        open_spot_flag = true;
                        open_row = r;
                        open_col = c;
                    }
                } else if open_spot_flag {
                    cur_state.current=merge_squares(Coordinate{x:open_col,y:open_row}, Coordinate{x:c,y:r}, cur_state.current.0[r].0[c], cur_state.current);
                    open_spot_flag = true;
                    match dir {
                        Direction::Up => { open_row = open_row+1; },
                        Direction::Down => {open_row =open_row-1; },
                        Direction::Left => { open_col =open_col+1; },
                        Direction::Right => { open_col =open_col-1; },
                    }
                } 
            }
          }
          cur_state
    }
    pub fn init_state() -> GameFrame {
        let mut frame: GameFrame = GameFrame(Default::default());
        let mut rng = rand::thread_rng();
        let mut coord:Coordinate = Coordinate{
            x: 0,
            y:0
        };
        for _b in 0 ..=1 {
            coord.x = rng.gen_range(0..=3);
            coord.y = rng.gen_range(0..=3);
            while frame.0[coord.y].0[coord.x] !=0 {
                coord.x = rng.gen_range(0..=3);
                coord.y = rng.gen_range(0..=3);
            }
            frame.0[coord.y].0[coord.x] = if rng.gen_range(0..10)!=0 { 2} else { 4 };
        }
        frame
    }
    fn gen_square(cur_frame: GameFrame, count: u8) -> GameFrame {
        let mut frame = cur_frame;
        let mut rng = rand::thread_rng();
        let mut coord:Coordinate = Coordinate{..Default::default()};
        for _b in 0 ..count {
            coord.x = rng.gen_range(0..=3);
            coord.y = rng.gen_range(0..=3);
            while frame.0[coord.y].0[coord.x] !=0 {
                coord.x = rng.gen_range(0..=3);
                coord.y = rng.gen_range(0..=3);
            }
            frame.0[coord.y].0[coord.x] = if rng.gen_range(0..10)!=0 { 2} else { 4 };
        }
        frame
    }
    pub fn new_game(init_state: GameFrame) -> GameState {
        GameState {
            current: init_state,
            prev: GameFrame(Default::default()),
            moves: 0
        }
    }

    pub fn check_valid(move_dir: Direction, state: &GameState)->bool{
        let mut nonzero_val: u32;
        let mut check_value: u32;
        let mut c: usize;
        let mut r: usize;

        let rev: bool = move_dir == Direction::Left || move_dir == Direction::Up;
        let outer_loop_range = match create_range(false, 4) {
            Ok(range) => range,
            Err(error) => panic!("Problem Creating Range, {:?}", error),
        };
        // TODO : Added case where adjacent numbers are same.
        for outer in outer_loop_range {
            nonzero_val = 0;
            let inner_loop_range = match create_range(rev, 4) {
                Ok(range) => range,
                Err(error) => panic!("Problem Creating Range, {:?}", error),
            };
            for inner in inner_loop_range {
                if move_dir == Direction::Right || move_dir == Direction::Left {
                    r = outer;
                    c = inner;
                }else{
                    r = inner;
                    c = outer;
                }
                check_value = state.current.0[r].0[c];
                if check_value != 0 {
                    if nonzero_val == 0 || nonzero_val != check_value {
                        nonzero_val = check_value;
                    } else if check_value == nonzero_val {
                        return true;
                    } 
                } else if nonzero_val != 0 {
                    return true;
                }

            }
        }
        false
    }
    fn merge_squares(main_cell: Coordinate, merge_cell: Coordinate, cell_value: u32, cur_frame: GameFrame) -> GameFrame  {
        let mut frame = cur_frame;
        frame.0[main_cell.y].0[main_cell.x] = cell_value;
        frame.0[merge_cell.y].0[merge_cell.x] = 0;
        frame
    }
    
    pub fn process_move(move_dir: Direction, state: GameState) -> GameState {
        
        let mut cur_state = state;
        
        if !check_valid(move_dir, &cur_state) {
            return cur_state;
        }
        //move step 1
        cur_state = move_squares(move_dir, cur_state);
        //merge step
        cur_state = merge_adjacent(move_dir, cur_state);
        //move step 2
        cur_state = move_squares(move_dir, cur_state);
        cur_state.current= gen_square(cur_state.current,1);
        cur_state.moves+=1;
        cur_state
    } 
