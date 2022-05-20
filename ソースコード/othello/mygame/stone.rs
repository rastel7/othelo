#[derive(Copy, Clone)]
pub enum Stone {
    White,
    Black,
    Blank,
}
impl Stone {
    pub fn reversestone(&mut self) {
        /*
            自身をひっくり返す
        */
        match self {
            Stone::White => *self = Stone::Black,
            Stone::Black => *self = Stone::White,
            _ => {}
        };
    }
    pub fn return_reverse_color(&self) -> Stone {
        /*
            自身を反対色を返す
        */
        match self {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
            Stone::Blank => Stone::Blank,
        }
    }
}
impl std::fmt::Debug for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Stone::Black => write!(f, "黒"),
            Stone::White => write!(f, "白"),
            Stone::Blank => write!(f, "・"),
        }
    }
}
