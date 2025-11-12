pub trait ShowInfo {
    fn show_info(&self);
}

pub struct Undergrad {
    pub gpa: f64,
    pub major: String,
}

pub struct Grad {
    pub gpa: f64,
    pub major: String,
    pub thesis: String,
}

impl ShowInfo for Undergrad {
    fn show_info(&self) {
        println!("Major: {}", self.major);
        println!("GPA: {:.2}", self.gpa);
    }
}

impl ShowInfo for Grad {
    fn show_info(&self) {
        println!("Major: {}", self.major);
        println!("GPA: {:.2}", self.gpa);
        println!("Thesis: {}", self.thesis);
    }
}

pub struct Enrollment<T: ShowInfo> {
    pub students: Vec<T>,
}

impl<T: ShowInfo> ShowInfo for Enrollment<T> {
    fn show_info(&self) {
        for s in &self.students {
            s.show_info();
        }
    }
}

pub fn main() {
    let undergrads = Enrollment {
        students: vec![
            Undergrad {
                gpa: 3.5,
                major: "Computer Science".to_string(),
            },
            Undergrad {
                gpa: 3.8,
                major: "Mathematics".to_string(),
            },
        ],
    };

    let grads = Enrollment {
        students: vec![
            Grad {
                gpa: 3.9,
                major: "Chemistry".to_string(),
                thesis: "Energy consumed to break an atom".to_string(),
            },
        ],
    };


    undergrads.show_info();
    grads.show_info();
}