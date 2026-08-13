struct File<State>{
    state:State,
}

struct Closed;
struct Open;
impl File<Closed>{
    fn open(self)->File<Open>
    {
        println!("Opening the file");
        File{state:Open}
    }
}
impl File<Open>{
    fn read(&self){
        println!("Reading the file");
    }
    fn write(&self){
        println!("Writing the file");
    }
    fn close(self)->File<Closed>{
        println!("closing the file");
        File{state:Closed}
    }
}

fn main(){
    let closed_file = File{state:Closed};
    let open_file = closed_file.open();
    open_file.read();
    open_file.write("Hello World");
    let closed_again = open_file.close();
}