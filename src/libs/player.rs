use std::{fmt::Display,str::FromStr, string::ParseError, collections::HashMap};

use crate::libs::position::Position;

use super::attributes::Attribute;
#[derive(Debug)]
pub struct Player {
    id:i64,
    name:String,
    surname:String,
    position: Position,
    attributes:Vec<String>,
}
impl Player {
   pub fn new(id:i64, name:String, surname:String, position:Position,attributes: Vec<String>)->Self{
        Self { id, name, surname, position,attributes}
   }
    pub fn calc(&self)->HashMap<Attribute,i8>{
        let mut hm:HashMap<Attribute,i8> = HashMap::new();
        self.attributes.iter().enumerate().for_each(|(index,i)|{
            let t:i8 = i.parse().unwrap();
            let _ = match index {
               0 => hm.insert(Attribute::Crossing, t),
               1 => hm.insert(Attribute::Dribbling, t),
               2 => hm.insert(Attribute::Passing, t),
               3 => hm.insert(Attribute::Shooting, t),
               4 => hm.insert(Attribute::Tackle, t),
               5 => hm.insert(Attribute::Technique, t),
               6 => hm.insert(Attribute::Aggression, t),
               7 => hm.insert(Attribute::Creative, t),
               8 => hm.insert(Attribute::Decision, t),
               9 => hm.insert(Attribute::Leadership, t),
               10 => hm.insert(Attribute::Movement, t),
               11 => hm.insert(Attribute::Position, t),
               12 => hm.insert(Attribute::Teamwork, t),
               _ => panic!("Out of attributes range")
            };
        });
        hm
    }
    pub fn getattributes(&self)->Vec<String>{
        self.attributes.clone()
    }
}
impl Display for Player {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f,"{}. {} {} | {}",self.id,self.name,self.surname,self.position)
   } 
}
impl FromStr for Player{
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let temp:Vec<&str> = s.split(",").collect();
        let id:i64 = temp[0].parse().unwrap();
        let name:String = temp[1].to_string();
        let surname:String = temp[2].to_string();
        let position:Position = temp[3].parse().unwrap();
        let attributes : Vec<String> = (&temp[4..]).to_vec().iter().map(|i| i.trim_end_matches("\r").to_string()).collect();
        
        Ok(Player{
            id,name,surname,position,attributes
        })
    } 
}
