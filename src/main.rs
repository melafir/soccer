use soccer::libs::{csvparser, roles::PositionRoles};
fn main() {
    let f = csvparser::toplayer();
    let pos = PositionRoles::make(); 
    for i in f{
        println!("{}",i.surname);
        let arr = pos.get(&i.position).unwrap();
        let calc = i.calcroles();
        arr.iter().for_each(|r|{
            let (rat,av) = calc.get(&r).unwrap();
            println!("{:?} -> {} | {}",r,rat,av);
        });
        println!("______________________________")
    }
}
