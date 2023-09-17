use std::fs::read_to_string;
use std::path::Path;
use crate::libs::player::Player;
use crate::libs::position::Position;
fn csvparser()->Vec<String> {
    let path = Path::new("forfm.csv");
    let mut t:Vec<String> = Vec::new();
    match read_to_string(path){
        Err(_)=>panic!("Couldn't read file"),
        Ok(f)=>{
            let mut temp:Vec<&str> = f.split("\n").collect();
            for i in temp.iter_mut().skip(1){
               t.push(i.to_owned()); 
            }
        }
    }
    t
}
pub fn toplayer()->Vec<Player>{
    let arr = csvparser();
    let mut t:Vec<Player> =Vec::new();
    for i in arr{
        let temp:Vec<&str> = i.split(",").collect();
        let id:i64 = temp[0].parse().unwrap();
        let name = temp[1].to_string();
        let surname = temp[2].to_string();
        let position:Position = temp[3].parse().unwrap();
        t.push(Player::new(id,name,surname,position));
    }
    t
}
