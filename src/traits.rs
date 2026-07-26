struct Circle{
    radius:f64,
}

trait Shape{
    fn area_circle(&self)->f64;
}

impl Shape for Circle{
    fn area_circle(&self)->f64 {
        3.14*self.radius*self.radius
    }
}

fn main(){
    let circle = Circle{radius:5.0};
    circle.area_circle();
}


struct Seminar {
    title: String,
    speaker: String,
    location: String,
}

struct Workshop {
    title: String,
    instructor: String,
    duration: u32, // in hours
}

trait Course {
    fn get_overview(&self) -> String;
}

impl Course for Seminar {
    fn get_overview(&self) -> String {
        format!(
            "Seminar: {}\nSpeaker: {}\nLocation: {}",
            self.title, self.speaker, self.location
        )
    }
}

impl Course for Workshop {
    fn get_overview(&self) -> String {
        format!(
            "Workshop: {}\nInstructor: {}\nDuration: {} hours",
            self.title, self.instructor, self.duration
        )
    }
}

fn main() {
    let seminar = Seminar {
        title: String::from("Rust for Beginners"),
        speaker: String::from("Alice"),
        location: String::from("Conference Hall"),
    };

    let workshop = Workshop {
        title: String::from("Advanced Rust"),
        instructor: String::from("Bob"),
        duration: 6,
    };

    println!("{}", seminar.get_overview());
    println!();
    println!("{}", workshop.get_overview());
}