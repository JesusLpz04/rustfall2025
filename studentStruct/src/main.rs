struct Student{
    name:String,
    major: String
}
impl Student{
    fn new(name:String, major:String)->Student{
        Student{
            name:name,
            major:major
        }
    }
    fn get_major(&self)-> &String{
        return &self.major
    }
    fn set_major(&mut self, new_major: String){
        self.major=new_major
    } 
}


fn main() {
    let mut my_student = Student::new("turner".to_string(),"cs".to_string());
    println!("{}",my_student.name);
    println!("{}",my_student.major);
    my_student.set_major("Eng".to_string());
    println!("{}",my_student.get_major());
    


}
