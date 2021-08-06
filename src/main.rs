#[allow(dead_code)]
// Create model
struct Student {
    roll_no: String,
    name: String,
}

impl Student {
    fn new() -> Self {
        Self {
            roll_no: String::new(),
            name: String::new(),
        }
    }
    fn get_roll_no(&self) -> &str {
        self.roll_no.as_str()
    }
    fn set_roll_no(&mut self, roll_no: &str) {
        self.roll_no = roll_no.into();
    }
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }
}

// Create view
struct StudentView;
impl StudentView {
    fn print_student_details(&self, name: &str, roll_no: &str) {
        println!("Student:");
        println!("Name: {}", name);
        println!("Roll no.: {}", roll_no);
    }
}

// Create controller
struct StudentController {
    model: Student,
    view: StudentView,
}

impl StudentController {
    fn new(model: Student, view: StudentView) -> Self {
        Self { model, view }
    }
    fn set_student_name(&mut self, name: &str) {
        self.model.set_name(name);
    }
    fn get_student_name(&self) -> &str {
        self.model.get_name()
    }
    fn set_student_roll_no(&mut self, roll_no: &str) {
        self.model.set_roll_no(roll_no);
    }
    fn get_student_roll_no(&self) -> &str {
        self.model.get_roll_no()
    }
    fn update_view(&self) {
        self.view
            .print_student_details(self.get_student_name(), self.get_student_roll_no());
    }
}

fn retrieve_student_from_db() -> Student {
    let mut student = Student::new();
    student.set_name("Robert");
    student.set_roll_no("10");
    student
}

fn main() {
    let model: Student = retrieve_student_from_db();

    let view: StudentView = StudentView;

    let mut controller: StudentController = StudentController::new(model, view);

    controller.update_view();

    controller.set_student_name("John");
    controller.set_student_roll_no("20");

    controller.update_view();
}
