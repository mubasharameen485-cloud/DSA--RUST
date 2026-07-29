struct Student {
    id: i32,
    name: String,
    age: u8,
    marks: f64,
    passed: bool,
}

impl Student {
    fn show_info(&self) {
        println!("ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Marks: {}", self.marks);
        println!("Passed: {}", self.passed);
    }

    fn get_name(&self) {
        println!("Student Name: {}", self.name);
    }

    fn check_result(&self) {
        if self.passed {
            println!("{} has passed", self.name);
        } else {
            println!("{} has failed", self.name);
        }
    }

    fn add_marks(&mut self, extra: f64) {
        self.marks += extra;
        println!("New Marks of {}: {}", self.name, self.marks);
    }

    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
        println!("Updated Name: {}", self.name);
    }
}

fn main() {
    let mut s1 = Student {
        id: 1,
        name: String::from("Ali"),
        age: 20,
        marks: 85.5,
        passed: true,
    };

    let s2 = Student {
        id: 2,
        name: String::from("Ahmed"),
        age: 21,
        marks: 70.0,
        passed: true,
    };

    let s3 = Student {
        id: 3,
        name: String::from("Sara"),
        age: 19,
        marks: 45.5,
        passed: false,
    };

    let s4 = Student {
        id: 4,
        name: String::from("Zain"),
        age: 22,
        marks: 90.0,
        passed: true,
    };

    let s5 = Student {
        id: 5,
        name: String::from("Hina"),
        age: 20,
        marks: 60.5,
        passed: true,
    };

    s1.show_info();
    s2.get_name();
    s3.check_result();
    s1.add_marks(5.0);
    s1.change_name(String::from("Ali Khan"));

    s4.show_info();
    s5.check_result();
}