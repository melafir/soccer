use std::{fmt::Display, collections::HashMap};

use crate::libs::position::Position;

use super::{attributes::Attribute, roles::{Roles, AttributesRole}};
pub struct Player {
    id:i64,
    name:String,
    surname:String,
    position: Position,
    attributes:HashMap<Attribute,i8>,
    roles: HashMap<Roles,i8>
}
impl Player {
   pub fn new(id:i64, name:String, surname:String, position:Position,attributes: HashMap<Attribute,i8>,roles:HashMap<Roles,i8>)->Self{
        Self { id, name, surname, position,attributes,roles}
   }
    pub fn calc(&mut self,ar: Vec<i8>){
        let ar = AttributesRole::make();
        for (_role,val) in self.roles.iter_mut(){
            let t = ar.get(&_role).unwrap();
            for (_attr,vala) in self.attributes{
               *val+=vala; 
            }

        }
    }
    pub fn getattributes(&mut self, ar:Vec<i8>){
        self.attributes.insert(Attribute::Crossing,ar[0]);

    }
}
impl Display for Player {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f,"{}. {} {} | {}",self.id,self.name,self.surname,self.position)
   } 
}
