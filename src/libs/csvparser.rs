use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use std::io::{self, Write};

use crate::libs::player::Player;
use crate::libs::roles::AttributesRole;

use super::roles::Roles;

fn csvparser(f:&str)->Vec<String> {
    let path = Path::new(f);
    let mut t:Vec<String> = Vec::new();
    match read_to_string(path){
        Err(e)=>panic!("{}",e),
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
pub fn choose(){
    let mut roles_array :Vec<Roles> = Vec::new();
    let ra = AttributesRole::make();

    for (r,_) in ra.iter(){
        roles_array.push(r.clone());
    }
    loop{
        roles_array.iter().enumerate().for_each(|(index,r)|{
            if index%3==0 {
                println!()
            }
            print!("{:>2} : {:>15} | ",index +1,r);
        });
        println!();
        print!("Choose role to show ranking: ");
        io::stdout().flush().unwrap();
        let mut chrole = String::new();
        io::stdin().read_line(&mut chrole).expect("Wrong role!");
        let cr:usize = chrole.trim().parse().expect("Faild to parse role");
        println!();
        if cr==0{break;}
        let mut f = toplayer("forfm.csv");
        let d = toplayer("duble.csv");
        f.extend(d);
        let role = roles_array[cr-1];
        let out = pr_by_roles(role, f);
        println!("{}",role);
        out[0..10].iter().enumerate().for_each(|(index,(name,av))|{
            if index%3 == 0{
                println!()
            }
            print!("{:<15} -> {:>5}   |   ",name,av);
        });
        println!();
    }
}
fn choose_by_role(ar:Vec<Player>) -> Vec<(String,Roles,f32)>{
    let mut out : Vec<(String,Roles,f32)> = Vec::new();
    ar.iter().for_each(|i|{
        let cal = i.calcroles();
        cal.iter().for_each(|(r,(_,av))|{
            out.push((i.surname.clone(),r.clone(),*av));
        })
    });
    out
}
pub fn pr_by_roles(r:Roles,ar:Vec<Player>) -> Vec<(String,f32)>{
    let v = choose_by_role(ar);
    let mut out:Vec<(String, f32)> = Vec::new();
    v.iter().for_each(|(name,role,av)|{
        if r==*role {
         out.push((name.clone(),*av))
        }
    });
    out.sort_by(|a,b|b.1.partial_cmp(&a.1).unwrap());
    out
}

