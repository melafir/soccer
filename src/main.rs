use soccer::libs::csvparser;
fn main() {
    let f = csvparser::toplayer();
    for i in f{
        println!("{:?}",i.calc());
    }
}
