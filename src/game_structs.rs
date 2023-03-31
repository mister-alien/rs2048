// import fmt to make available
    use std::fmt;
    use rand::Rng;
    use itertools::Itertools;
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
    pub struct Row([u32; 4]);
    pub struct GameFrame([Row; 4]);
    pub struct GameState {
        pub current: GameFrame,
        prev: GameFrame,
        moves: u32,
    }
    // CREATES range UNTIL size (range of size "size" )
    pub fn create_range(
        rev: bool,  size: usize) -> itertools::Either<impl Iterator<Item = usize>, impl Iterator<Item = usize>> {
        if !rev {
            itertools::Either::Left(0..size)
        } else {
            itertools::Either::Right((0..size).rev())
        }
    }
    pub fn move_squares(dir: Direction, state: GameState) -> GameState {
        let mut nonzero_val: u32 = 0;
        let mut open_spot_flag: bool = false;
        let mut cur_state = state;
        let mut open_col: usize = 0;
        let mut open_row: usize = 0;
        let mut c: usize=0;
        let mut r: usize=0;
        let rev: bool = dir == Direction::Right || dir == Direction::Down;
        for outer in create_range(false, 4) {
            //check_cell = false;
            for inner in create_range(rev, 4) {
                open_spot_flag = false;
                if dir == Direction::Up || dir == Direction::Left {
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
            
                            Direction::Up => { open_row +=1; },
                            Direction::Down => {open_row -=1; },
                            Direction::Left => { open_col +=1},
                            Direction::Right => { open_col -=1},
                            _ => {
                                println!("Woah there pal");
                            }

                        }
                    } else {
                        
                    }
            }
          }
          cur_state
    }
    pub fn init_state() -> GameFrame {
        let mut frame: GameFrame = GameFrame(
            [Row([0,0,0,0]),
            Row([0,0,0,0]),
            Row([0,0,0,0]),
            Row([0,0,0,0])]);
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
        let mut coord:Coordinate = Coordinate{
            x: 0,
            y:0
        };
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
            prev: GameFrame([Row([0,0,0,0]),
            Row([0,0,0,0]),
            Row([0,0,0,0]),
            Row([0,0,0,0]),
            ]),
            moves: 0
        }
    }

    pub fn check_valid(move_dir: Direction, state: &GameState)->bool{
        let mut nonzero_val: u32 = 0;
        //let mut check_cell: bool = false;
        //let mut valid_move: bool = false;
// TODO : Added case where adjacent numbers are same.
        match move_dir {
            
            Direction::Up => {
                for c in 0..=3 {
                  nonzero_val = 0;
                  //check_cell = false;
                  for r in (0..=3).rev() {
                    if state.current.0[r].0[c]!=0 {
                      if nonzero_val == 0 || nonzero_val != state.current.0[r].0[c] {
                        nonzero_val = state.current.0[r].0[c];
                      }else if state.current.0[r].0[c] == nonzero_val{
                        return true;
                      } else {
                        
                      }
                    } else if nonzero_val != 0 {
                        return true;
                    }
                  }
                }
            },
            Direction::Down => {
                for c in 0..=3 {
                    nonzero_val = 0;
                    //check_cell = false;
                    for r in 0..=3 {
                      if state.current.0[r].0[c]!=0 {
                        if nonzero_val == 0 || nonzero_val != state.current.0[r].0[c] {
                          nonzero_val = state.current.0[r].0[c];
                        }else if state.current.0[r].0[c] == nonzero_val{
                          return true;
                        } else {
                          
                        }
                      } else if nonzero_val != 0 {
                          return true;
                      }
                    }
                  }
            },
            Direction::Left => {
                for r in 0..=3 {
                    nonzero_val = 0;
                    //check_cell = false;
                    for c in (0..=3).rev() {
                      if state.current.0[r].0[c]!=0 {
                        if nonzero_val == 0 || nonzero_val != state.current.0[r].0[c] {
                          nonzero_val = state.current.0[r].0[c];
                        }else if state.current.0[r].0[c] == nonzero_val{
                          return true;
                        } else {
                          
                        }
                      } else if nonzero_val != 0 {
                          return true;
                      }
                    }
                  }
            },
            Direction::Right => {
                for r in 0..=3 {
                    nonzero_val = 0;
                    //check_cell = false;
                    for c in 0..=3 {
                      if state.current.0[r].0[c]!=0 {
                        if nonzero_val == 0 || nonzero_val != state.current.0[r].0[c] {
                          nonzero_val = state.current.0[r].0[c];
                        }else if state.current.0[r].0[c] == nonzero_val{
                          return true;
                        } else {
                          
                        }
                      } else if nonzero_val != 0 {
                          return true;
                      }
                    }
                  }
            },
            //_ => println!("Invalid Move"),

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
        let mut check_value: u32 = 0;
        let mut open_spot: usize = 0;
        let mut open_spot_flag: bool = false;
        let mut cur_state = state;
        let mut merge_val: u32 = 0;
        let mut merge_spot: usize = 0;
        //let cur_dir: Direction = move_dir;
        let mut changed: bool = false;
        let mut valid_merge: bool = true;
        let mut valid_move: bool = false;
        let mut move_to: usize = 0;
        if !check_valid(move_dir, &cur_state) {
            return cur_state;
        }
        //let mut merge_coord: Coordinate = Coordinate{x: 0, y:0};
        match move_dir {
            
        Direction::Up => {
            for c in 0..=3 {
                open_spot_flag = false;
                //move step
                for r in 0..=3 {
                    if cur_state.current.0[r].0[c] == 0 {
                        if !open_spot_flag {
                            open_spot_flag = true;
                            open_spot = r;
                        }
                    } else if open_spot_flag {
                        cur_state.current=merge_squares(Coordinate{x:c,y:open_spot}, Coordinate{x:c,y:r}, cur_state.current.0[r].0[c], cur_state.current);
                        open_spot_flag = true;
                        open_spot = open_spot+1;
                    } else {
                        
                    }
                }
                valid_merge = false;
                merge_val = 0;
                merge_spot = 0;
                //merge step
                for r in 0..=3 {
                    check_value = cur_state.current.0[r].0[c];
                    if check_value ==0 {
                        valid_merge = false;
                    } else if check_value >0 && !valid_merge {
                        merge_spot = r;
                        merge_val = check_value;
                        valid_merge = true;
                    } else if check_value >0 && valid_merge  {
                        if check_value == merge_val {
                            cur_state.current=merge_squares(Coordinate{x:c,y:merge_spot}, Coordinate{x:c,y:r}, merge_val*2, cur_state.current);
                            valid_merge = false;
                        } else {
                            merge_val = check_value;
                            merge_spot = r;
                        }

                    }
                }
                //move step 2
                open_spot_flag = false;
                //move step
                for r in 0..=3 {
                    if cur_state.current.0[r].0[c] == 0 {
                        if !open_spot_flag {
                            open_spot_flag = true;
                            open_spot = r;
                        }
                    } else if open_spot_flag {
                        cur_state.current=merge_squares(Coordinate{x:c,y:open_spot}, Coordinate{x:c,y:r}, cur_state.current.0[r].0[c], cur_state.current);
                        open_spot_flag = true;
                        open_spot = open_spot+1;
                    } else {
                        
                    }
                }
            }
            cur_state.current= gen_square(cur_state.current,1);
            cur_state.moves+=1;
        },
            Direction::Left => {
                for r in 0..=3 {
                    open_spot_flag = false;
                    //move step
                    for c in 0..=3 {
                        if cur_state.current.0[r].0[c] == 0 {
                            if !open_spot_flag {
                                open_spot_flag = true;
                                open_spot = c;
                            }
                        } else if open_spot_flag {
                            cur_state.current=merge_squares(Coordinate{x:open_spot,y:r}, Coordinate{x:c,y:r}, cur_state.current.0[r].0[c], cur_state.current);
                            open_spot_flag = true;
                            open_spot = open_spot+1;
                        } else {
                            
                        }
                    }
                    valid_merge = false;
                    merge_val = 0;
                    merge_spot = 0;
                    //merge step
                    for c in 0..=3 {
                        check_value = cur_state.current.0[r].0[c];
                        if check_value ==0 {
                            valid_merge = false;
                        } else if check_value >0 && !valid_merge {
                            merge_spot = c;
                            merge_val = check_value;
                            valid_merge = true;
                        } else if check_value >0 && valid_merge  {
                            if check_value == merge_val {
                                cur_state.current=merge_squares(Coordinate{x:merge_spot,y:r}, Coordinate{x:c,y:r}, merge_val*2, cur_state.current);
                                valid_merge = false;
                            } else {
                                merge_val = check_value;
                                merge_spot = c;
                            }
    
                        }
                    }
                    //move step 2
                    open_spot_flag = false;
                    //move step
                    for c in 0..=3 {
                        if cur_state.current.0[r].0[c] == 0 {
                            if !open_spot_flag {
                                open_spot_flag = true;
                                open_spot = c;
                            }
                        } else if open_spot_flag {
                            cur_state.current=merge_squares(Coordinate{x:open_spot,y:r}, Coordinate{x:c,y:r}, cur_state.current.0[r].0[c], cur_state.current);
                            open_spot_flag = true;
                            open_spot = open_spot+1;
                        } else {
                            
                        }
                    }
                }
                cur_state.current= gen_square(cur_state.current,1);
                cur_state.moves+=1;},
              Direction::Right => {
                for r in 0..=3 {
                    open_spot_flag = false;
                    //move step
                    for c in (0..=3).rev() {
                        if cur_state.current.0[r].0[c] == 0 {
                            if !open_spot_flag {
                                open_spot_flag = true;
                                open_spot = c;
                            }
                        } else if open_spot_flag {
                            cur_state.current=merge_squares(Coordinate{x:open_spot,y:r}, Coordinate{x:c,y:r}, cur_state.current.0[r].0[c], cur_state.current);
                            open_spot_flag = true;
                            open_spot = open_spot-1;
                        } else {
                            
                        }
                    }
                    valid_merge = false;
                    merge_val = 0;
                    merge_spot = 0;
                    //merge step
                    for c in (0..=3).rev() {
                        check_value = cur_state.current.0[r].0[c];
                        if check_value ==0 {
                            valid_merge = false;
                        } else if check_value >0 && !valid_merge {
                            merge_spot = c;
                            merge_val = check_value;
                            valid_merge = true;
                        } else if check_value >0 && valid_merge  {
                            if check_value == merge_val {
                                cur_state.current=merge_squares(Coordinate{x:merge_spot,y:r}, Coordinate{x:c,y:r}, merge_val*2, cur_state.current);
                                valid_merge = false;
                            } else {
                                merge_val = check_value;
                                merge_spot = c;
                            }
    
                        }
                    }
                    //move step 2
                    open_spot_flag = false;
                    //move step
                    for c in (0..=3).rev() {
                        if cur_state.current.0[r].0[c] == 0 {
                            if !open_spot_flag {
                                open_spot_flag = true;
                                open_spot = c;
                            }
                        } else if open_spot_flag {
                            cur_state.current=merge_squares(Coordinate{x:open_spot,y:r}, Coordinate{x:c,y:r}, cur_state.current.0[r].0[c], cur_state.current);
                            open_spot_flag = true;
                            open_spot = open_spot-1;
                        } else {
                            
                        }
                    }
                }
                cur_state.current= gen_square(cur_state.current,1);
                cur_state.moves+=1;},
            Direction::Down => {
                for c in 0..=3 {
                    open_spot_flag = false;
                    //move step
                    for r in (0..=3).rev() {
                        if cur_state.current.0[r].0[c] == 0 {
                            if !open_spot_flag {
                                open_spot_flag = true;
                                open_spot = r;
                            }
                        } else if open_spot_flag {
                            cur_state.current=merge_squares(Coordinate{x:c,y:open_spot}, Coordinate{x:c,y:r}, cur_state.current.0[r].0[c], cur_state.current);
                            open_spot_flag = true;
                            open_spot = open_spot-1;
                        } else {
                            
                        }
                    }
                    valid_merge = false;
                    merge_val = 0;
                    merge_spot = 0;
                    //merge step
                    for r in (0..=3).rev() {
                        check_value = cur_state.current.0[r].0[c];
                        if check_value ==0 {
                            valid_merge = false;
                        } else if check_value >0 && !valid_merge {
                            merge_spot = r;
                            merge_val = check_value;
                            valid_merge = true;
                        } else if check_value >0 && valid_merge  {
                            if check_value == merge_val {
                                cur_state.current=merge_squares(Coordinate{x:c,y:merge_spot}, Coordinate{x:c,y:r}, merge_val*2, cur_state.current);
                                valid_merge = false;
                            } else {
                                merge_val = check_value;
                                merge_spot = r;
                            }
    
                        }
                    }
                    //move step 2
                    open_spot_flag = false;
                    //move step
                    for r in (0..=3).rev() {
                        if cur_state.current.0[r].0[c] == 0 {
                            if !open_spot_flag {
                                open_spot_flag = true;
                                open_spot = r;
                            }
                        } else if open_spot_flag {
                            cur_state.current=merge_squares(Coordinate{x:c,y:open_spot}, Coordinate{x:c,y:r}, cur_state.current.0[r].0[c], cur_state.current);
                            open_spot_flag = true;
                            open_spot = open_spot-1;
                        } else {
                            
                        }
                    }
                }
                cur_state.current= gen_square(cur_state.current,1);
                cur_state.moves+=1;
                },
            _ => println!("Direction was wrong!"),
        }
        cur_state
    } 
    impl fmt::Display for Row {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"[{},{},{},{}]", self.0[0],self.0[1],self.0[2],self.0[3])
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
    impl fmt::Display for GameState {

        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"Move: {}\n{}", self.moves, 
                self.current)
        }
    }
