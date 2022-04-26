use crate::mygame::stone::Stone;
#[derive(Debug)]
pub struct User {
    pub now: Stone,
}
impl User {
    pub fn nextuser(&mut self) {
        match self.now {
            Stone::Black => self.now = Stone::White,
            Stone::White => self.now = Stone::Black,
            Stone::Blank => eprintln!("Blnkユーザーになっています！！"),
        }
    }
}
