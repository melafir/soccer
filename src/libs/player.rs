use std::fmt::Display;

use crate::libs::position::Position;
pub struct Player {
    id:i64,
    name:String,
    surname:String,
    position: Position,
}
impl Player {
   pub fn new(id:i64, name:String, surname:String, position:Position)->Self{
        Self { id, name, surname, position}
   }
}
impl Display for Player {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f,"{}. {} {} | {}",self.id,self.name,self.surname,self.position)
   } 
}
