// import fmt to make available
    use std::fmt;
    use rand::Rng;

    // All possible game moves :)
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
        let mut check_cell: bool = false;
        let mut valid_move: bool = false;
// TODO : Add case where full row is full but there is adjacent (n+1) same number
// It seems to work in some dirs (down, right) but not others......
        match move_dir {
            
            Direction::Up => {
                for c in 0..=3 {
                  check_cell = false;
                  for r in (0..=3).rev() {
                    if state.current.0[r].0[c]!=0 {
                      check_cell = true;
                    } else {
                        if check_cell {
                            valid_move = true;
                        }
                    }
                  }
                }
            },
            Direction::Down => {
                for c in 0..4 {
                  check_cell = false;
                  for r in 0..4 {
                    if state.current.0[r].0[c]!=0 {
                      check_cell = true;
                    } else {
                        if check_cell {
                            valid_move = true;
                        }
                    }
                  }
                }
            },
            Direction::Left => {
                for r in 0..=3 {
                  check_cell = false;
                  for c in (0..=3).rev() {
                    if state.current.0[r].0[c]!=0 {
                      check_cell = true;
                    } else {
                        if check_cell {
                            valid_move = true;
                        }
                    }
                  }
                }
            },
            Direction::Right => {
                for r in 0..4 {
                  check_cell = false;
                  for c in 0..4 {
                    if state.current.0[r].0[c]!=0 {
                      check_cell = true;
                    } else {
                        if check_cell {
                            valid_move = true;
                        }
                    }
                  }
                }
            },
            //_ => println!("Invalid Move"),

        }
        valid_move
    }
    fn merge_squares(main_cell: Coordinate, merge_cell: Coordinate, cell_value: u32, cur_frame: GameFrame) -> GameFrame  {
        let mut frame = cur_frame;
        frame.0[main_cell.y].0[main_cell.x] = cell_value;
        frame.0[merge_cell.y].0[merge_cell.x] = 0;
        frame
    }
    
    pub fn process_move(move_dir: Direction, state: GameState) -> GameState {
        let mut check_value: u32 = 0;
        let mut cur_state = state;
        let mut changed: bool = false;
        let mut valid_merge: bool = true;
        //let mut merge_coord: Coordinate = Coordinate{x: 0, y:0};
        match move_dir {
            
            Direction::Up => {
              if check_valid(Direction::Up, &cur_state) {
              loop{
                  //merge pieces in the right direction.
                  //cur_state.prev = cur_state.current;
                  changed = false;
                for c in 0..=3 {
                    valid_merge = true;
                    for r in 0..=3 {
                        check_value = cur_state.current.0[r].0[c];
                        if check_value != 0{
                        for rest in r+1..=3 {
                            if cur_state.current.0[rest].0[c] !=0 {
                                if cur_state.current.0[rest].0[c] == check_value && valid_merge {
                                    valid_merge = false;
                                    changed = true;
                                    cur_state.current=merge_squares(Coordinate{x:c,y:r}, Coordinate{x:c,y:rest}, check_value*2, cur_state.current);
                                } else {
                                    break;
                                }
                            }
                        }
                      }
                    }
                }
                if !changed  {
                    break;
                }

              }
              // Move pieces to the correct side.
              loop {
                  changed = false;
                for c in 0..=3{
                    for r in 0..=3 {
                        if cur_state.current.0[r].0[c] == 0 {
                            for rest in r+1..=3 {
                                let val: u32 = cur_state.current.0[rest].0[c];
                                if val != 0{
                                    changed = true;
                                    let from: Coordinate = Coordinate {
                                        x: c, y: rest
                                    };
                                    let to: Coordinate = Coordinate {
                                        x: c, y: r
                                    };
                                    cur_state.current=merge_squares(to, from, val, cur_state.current);
                                }
                            }
                        }
                    }
                }
                if !changed {
                    break;
                }
              }
              cur_state.current= gen_square(cur_state.current,1);
              cur_state.moves+=1;
            }},
            Direction::Left => {
                if check_valid(Direction::Left, &cur_state) {
                loop{
                    //merge pieces in the right direction.
                    //cur_state.prev = cur_state.current;
                    changed = false;
                  for r in 0..=3 {
                      valid_merge = true;
                      for c in 0..=3 {
                          check_value = cur_state.current.0[r].0[c];
                          if check_value != 0{
                          for rest in c+1..=3 {
                              if cur_state.current.0[r].0[rest] !=0 {
                                  if cur_state.current.0[r].0[rest] == check_value && valid_merge {
                                      valid_merge = false;
                                      changed = true;
                                      cur_state.current=merge_squares(Coordinate{x:c,y:r}, Coordinate{x:rest,y:r}, check_value*2, cur_state.current);
                                  } else {
                                      break;
                                  }
                              }
                          }
                        }
                      }
                  }
                  if !changed  {
                      break;
                  }
  
                }
                // Move pieces to the correct side.
                loop {
                    changed = false;
                  for r in 0..=3{
                      for c in 0..=3 {
                          if cur_state.current.0[r].0[c] == 0 {
                              for rest in c+1..=3 {
                                  let val: u32 = cur_state.current.0[r].0[rest];
                                  if val != 0{
                                      changed = true;
                                      let from: Coordinate = Coordinate {
                                          x: rest, y: r
                                      };
                                      let to: Coordinate = Coordinate {
                                          x: c, y: r
                                      };
                                      cur_state.current=merge_squares(to, from, val, cur_state.current);
                                  }
                              }
                          }
                      }
                  }
                  if !changed {
                      break;
                  }
                }
                cur_state.current= gen_square(cur_state.current,1);
                cur_state.moves+=1;
              }},
              Direction::Right => {
                if check_valid(Direction::Right, &cur_state) {
                loop{
                    //merge pieces in the right direction.
                    //cur_state.prev = cur_state.current;
                    changed = false;
                  for r in 0..=3 {
                      valid_merge = true;
                      for c in (0..=3).rev() {
                          check_value = cur_state.current.0[r].0[c];
                          if check_value != 0{
                          for rest in (0..c).rev() {
                              if cur_state.current.0[r].0[rest] !=0 {
                                  if cur_state.current.0[r].0[rest] == check_value && valid_merge {
                                      valid_merge = false;
                                      changed = true;
                                      cur_state.current=merge_squares(Coordinate{x:c,y:r}, Coordinate{x:rest,y:r}, check_value*2, cur_state.current);
                                  } else {
                                      break;
                                  }
                              }
                          }
                        }
                      }
                  }
                  if !changed  {
                      break;
                  }
  
                }
                // Move pieces to the correct side.
                loop {
                    changed = false;
                  for r in 0..=3{
                      for c in (0..=3).rev() {
                          if cur_state.current.0[r].0[c] == 0 {
                              for rest in (0..c).rev() {
                                  let val: u32 = cur_state.current.0[r].0[rest];
                                  if val != 0{
                                      changed = true;
                                      let from: Coordinate = Coordinate {
                                          x: rest, y: r
                                      };
                                      let to: Coordinate = Coordinate {
                                          x: c, y: r
                                      };
                                      cur_state.current=merge_squares(to, from, val, cur_state.current);
                                  }
                              }
                          }
                      }
                  }
                  if !changed {
                      break;
                  }
                }
                cur_state.current= gen_square(cur_state.current,1);
                cur_state.moves+=1;
              }},
            Direction::Down => {
                if check_valid(Direction::Down, &cur_state) {
                loop{
                    //merge pieces in the right direction.
                    //cur_state.prev = cur_state.current;
                    changed = false;
                  for c in 0..=3 {
                      valid_merge = true;
                      for r in (0..=3).rev() {
                          check_value = cur_state.current.0[r].0[c];
                          if check_value != 0{
                          for rest in (0..r).rev() {
                              if cur_state.current.0[rest].0[c] !=0 {
                                  if cur_state.current.0[rest].0[c] == check_value && valid_merge {
                                      valid_merge = false;
                                      changed = true;
                                      cur_state.current=merge_squares(Coordinate{x:c,y:r}, Coordinate{x:c,y:rest}, check_value*2, cur_state.current);
                                  } else {
                                      break;
                                  }
                              }
                          }
                        }
                      }
                  }
                  if !changed  {
                      break;
                  }
  
                }
                // Move pieces to the correct side.
                loop {
                    changed = false;
                  for c in 0..=3{
                      for r in (0..=3).rev() {
                          if cur_state.current.0[r].0[c] == 0 {
                              for rest in (0..r).rev() {
                                  let val: u32 = cur_state.current.0[rest].0[c];
                                  if val != 0{
                                      changed = true;
                                      let from: Coordinate = Coordinate {
                                          x: c, y: rest
                                      };
                                      let to: Coordinate = Coordinate {
                                          x: c, y: r
                                      };
                                      cur_state.current=merge_squares(to, from, val, cur_state.current);
                                  }
                              }
                          }
                      }
                  }
                  if !changed {
                      break;
                  }
                }
                cur_state.current= gen_square(cur_state.current,1);
                cur_state.moves+=1;
              }},
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
