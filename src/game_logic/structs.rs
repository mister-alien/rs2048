    use std::fmt;
    use crate::game_logic::logic;
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
    pub struct Coordinate {pub x: usize, pub y: usize }
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
            GameState { current: logic::init_state(), 
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
