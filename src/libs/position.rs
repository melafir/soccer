use std::{str::FromStr, fmt::Display};

#[derive(Hash,PartialEq,Debug)]
pub enum Position {
   GK,DR,DL,DC,WBR,WBL,DMC,MC,ML,MR,AMC,AML,AMR,FL,FR,ST 
}
impl FromStr for Position {
    type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
       match s {
          "dc"=>Ok(Position::DC),
          "mc"=>Ok(Position::MC),
          "st"=>Ok(Position::ST),
          _ => Err(())
       }
   } 
}
impl Display for Position {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
         Self::GK => write!(f,"GK"),
         Self::DR => write!(f,"DR"),
         Self::DL => write!(f,"DL"),
         Self::DC => write!(f,"DC"),
         Self::DMC => write!(f,"DMC"),
         Self::MC => write!(f,"MC"),
         Self::ML => write!(f,"ML"),
         Self::MR => write!(f,"MR"),
         Self::AMR => write!(f,"AMR"),
         Self::WBR => write!(f,"WBR"),
         Self::WBL => write!(f,"WBL"),
         Self::AMC => write!(f,"AMC"),
         Self::AML => write!(f,"AML"),
         Self::FL => write!(f,"FL"),
         Self::FR => write!(f,"FR"),
         Self::ST => write!(f,"ST"),

       }
   } 
}
impl Eq for Position {}
