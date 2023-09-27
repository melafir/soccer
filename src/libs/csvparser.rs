use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use std::io;

use crate::libs::player::Player;

fn csvparser(f:&str)->Vec<String> {
    let path = Path::new(f);
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
pub fn toplayer(f:&str)->Vec<Player>{
    let arr = csvparser(f);
    let mut t:Vec<Player> =Vec::new();
    for i in arr{
        t.push(Player::from_str(i.as_str()).unwrap());
    }
    t
}
pub fn choose()-> Vec<Player>{
    print!("Choose main[1] or double[2]: ");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Choose main or double: ");
    let c:u8 = inp.trim().parse().unwrap();
    println!();
    if c==1{
       toplayer("forfm.csv") 
    }else if c==2 {
       toplayer("duble.csv") 
    }else{
        panic!("Choose wrong")
    }


}
