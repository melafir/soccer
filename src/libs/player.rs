use std::{fmt::Display,str::FromStr, string::ParseError, collections::HashMap};

use crate::libs::position::Position;
use super::{attributes::Attribute, roles::{Roles, AttributesRole}};


#[derive(Debug)]
pub struct Player {
    id:i64,
    name:String,
    pub surname:String,
    pub position: Position,
    pub attributes:HashMap<Attribute,i8>,
}
impl Player {
    pub fn getattributes(&self){
        println!("{}",self.surname);
        for (k,v) in self.attributes.iter(){
            println!("{:?} {}",k,v)
        }
    }
    pub fn getattributesum(&self)->i16{
        let mut sum:i16 = 0;
        for v in self.attributes.values(){
           sum += *v as i16 ;
        }
        sum
    }
    pub fn calcroles(&self) ->HashMap<Roles,(i8,f32)>{
        let attr = AttributesRole::make();
        let mut hm:HashMap<Roles,(i8,f32)> = HashMap::new();
        for (r,va) in attr{
            let mut sum = 0;
            let av = va.len();
            for i in va{
               sum +=self.attributes.get(&i).unwrap();
            }

            hm.insert(r, (sum,((sum as f32/(av as f32))*100.00).round()/100.00));

        }
        hm
    }
    pub fn print_calcroles(&self)->Vec<String>{
        let mut ar:Vec<String> = Vec::new();
        self.calcroles().iter().for_each(|(r,(_,av))|{
            ar.push(format!("{} | {}",r,av))
        });
        ar
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
        let attributes_string : Vec<String> = (&temp[4..]).to_vec().iter().map(|i| i.trim_end_matches("\r").to_string()).collect();
        let mut attributes:HashMap<Attribute, i8> = HashMap::new();
        attributes_string.iter().enumerate().for_each(|(index,i)|{
            let t:i8 = i.parse().unwrap();
            let _ = match index {
               0 => attributes.insert(Attribute::Crossing, t),
               1 => attributes.insert(Attribute::Dribbling, t),
               2 => attributes.insert(Attribute::Passing, t),
               3 => attributes.insert(Attribute::Shooting, t),
               4 => attributes.insert(Attribute::Tackle, t),
               5 => attributes.insert(Attribute::Technique, t),
               6 => attributes.insert(Attribute::Aggression, t),
               7 => attributes.insert(Attribute::Creative, t),
               8 => attributes.insert(Attribute::Decision, t),
               9 => attributes.insert(Attribute::Leadership, t),
               10 =>attributes.insert(Attribute::Movement, t),
               11 =>attributes.insert(Attribute::Position, t),
               12 =>attributes.insert(Attribute::Teamwork, t),
               13 =>attributes.insert(Attribute::Pace, t),
               14 =>attributes.insert(Attribute::Stamina, t),
               15 =>attributes.insert(Attribute::Strength,t),
               16 =>attributes.insert(Attribute::Aero, t),
               _ => panic!("Out of attributes range")
            };
        });
        Ok(Player{
            id,name,surname,position,attributes
        })
    } 
}
