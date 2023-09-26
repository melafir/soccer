use soccer::libs::csvparser;
fn main() {
    let f = csvparser::toplayer();
    for i in f{
        println!("{} -> {}",i.surname,i.getattributesum());
        i.print_calcroles().iter().for_each(|i|println!("{i}"));
    }

}
