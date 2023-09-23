#[derive(Hash,PartialEq,Debug)]
pub enum Attribute {
   Crossing,
   Dribbling,
   Passing,
   Shooting,
   Tackle,
   Technique,
   Aggression,
   Creative,
   Decision,
   Leadership,
   Movement,
   Position,
   Teamwork,
   Pace,
   Stamina,
   Strength,
   Aero
}
impl Eq for Attribute {}
