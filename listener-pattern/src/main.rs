fn main() {
    let listener = StudentListener::new();
    let student = Student::new(1, String::from("gannon"), listener);
    student.learn();
}

trait Listener {
    fn learn(&self, event: Event);
}

#[derive(Debug, Clone)]
struct StudentListener;
impl StudentListener {
    fn new() -> StudentListener {
        StudentListener {}
    }
}
impl Listener for StudentListener {
    fn learn(&self, event: Event) {
        let student = event.student;
        println!("{}开始了学习", student.name);
    }
}

#[derive(Debug)]
struct Event {
    student: Student,
}

impl Event {
    fn new(student: Student) -> Event {
        Event { student }
    }
}

#[derive(Debug, Clone)]
struct Student {
    id: usize,
    name: String,
    listener: Option<StudentListener>,
}

impl Student {
    fn new(id: usize, name: String, listener: StudentListener) -> Student {
        Student {
            id,
            name,
            listener: Some(listener),
        }
    }
    fn learn(&self) {
        if let Some(_) = &self.listener {
            let event = Event::new(self.clone());
            let listener = self.listener.as_ref().unwrap();
            listener.learn(event);
        }
        println!("学习中,请勿打扰...");
    }
}
