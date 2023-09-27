use soccer::libs::csvparser;
fn main() {
    let f = csvparser::choose(); 
    for i in f{
        println!("{} -> {}",i.surname,i.getattributesum());
        i.calcroles().iter().enumerate().for_each(|(index,(r,(_,av)))|{
            print!("{}: {:5}   |",r,av);
            if (index+1)%3==0{
                println!();
            }
        });
        println!();
        println!();
    }
}
