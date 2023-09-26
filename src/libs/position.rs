use std::{str::FromStr, fmt::Display};

#[derive(Hash,PartialEq,Debug)]
pub enum Position {
   GK,DR,DL,DC,WBR,WBL,DMC,MC,ML,MR,AMC,AML,AMR,FL,FR,ST 
}
impl FromStr for Position {
    type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
       match s {
          "gk"=>Ok(Position::GK),
          "dr"=>Ok(Position::DR),
          "dl"=>Ok(Position::DL),
          "dc"=>Ok(Position::DC),
          "wbr"=>Ok(Position::WBR),
          "wbl"=>Ok(Position::WBL),
          "dmc"=>Ok(Position::DMC),
          "mc"=>Ok(Position::MC),
          "ml"=>Ok(Position::ML),
          "mr"=>Ok(Position::MR),
          "amc"=>Ok(Position::AMC),
          "aml"=>Ok(Position::AML),
          "fl"=>Ok(Position::FL),
          "fr"=>Ok(Position::FR),
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
