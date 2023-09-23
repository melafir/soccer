use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use crate::libs::player::Player;
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
        t.push(Player::from_str(i.as_str()).unwrap());
    }
    t
}
