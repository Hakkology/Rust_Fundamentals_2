#[derive(Debug, Clone)]
struct Student{
    name: String,
    age: u32,
    department: String
}

impl Student{
    
    fn add(name: String, age: u32, department: String) -> Student{
        
        let student = Student {        
            name: name,
            age: age,
            department: department};
        
        student
    }

    fn get_department(student: Student) -> String {
        student.department
    }

    fn get_department_2(&self) -> String {
        self.department.clone()
    }

    fn set_department(&mut self, department: String) {
        self.department = department;
    }

}

trait Crud{
    fn create(&self) -> Self; // Simulate creating a new instance based on the current one
    fn update(&mut self, name: Option<String>, age: Option<u32>, department: Option<String>); // Update the instance
    fn delete(&mut self); // Simulate deleting the instance
}

impl Crud for Student {

    fn create(&self) -> Self {
        Self {
            name: self.name.clone(),
            age: self.age,
            department: self.department.clone(),
        }
    }

    fn update(&mut self, name: Option<String>, age: Option<u32>, department: Option<String>) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(age) = age {
            self.age = age;
        }
        if let Some(department) = department {
            self.department = department;
        }
    }

    // Simulate deleting the instance by resetting its fields
    fn delete(&mut self) {
        self.name = String::new();
        self.age = 0;
        self.department = String::new();
    }
}
fn main(){

    let mut student = Student::add("Jane Doe".to_string(), 20, "Biology".to_string());
    println!("Original: {:?}", student);

    let new_student = student.create();
    println!("Created: {:?}", new_student);

    student.update(Some("John Doe".to_string()), Some(21), Some("Physics".to_string()));
    println!("Updated: {:?}", student);

    student.delete(); 
    println!("Deleted: {:?}", student);
}
