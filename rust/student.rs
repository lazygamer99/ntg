use std::collections::HashMap;
use std::io;
struct Student {
    name: String,
    age: u32,
    grade: u32,
}

fn main() {
    let mut student_record: HashMap<String, Student> = HashMap::new();
    
    loop {
        println!("Menu:");
        println!("1.To modify the student record");
        println!("2.To display all the records");
        println!("3.Quit");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed");
        match choice.trim().parse::<u32>() {
            Ok(1) => {
                let student = get_info();
                student_record.insert(student.name.clone(), student);
            }
            
            Ok(2) => {
                println!("Student details:");
                for (name,student) in &student_record {
                    println!("Name:{}, Age: {}, Grade:{}", name,student.age,student.grade );
                }
            }
            
            Ok(3) => {
                println!("Quiting program");
                break;
            }
            _ => println!("Enter valid choice"),
        }
    }
}

fn get_info() -> Student {
    println!("Enter student name:"); 
    let mut name = String::new(); 
    io::stdin().read_line(&mut name).expect("Failed to read line"); 
    let name = name.trim().to_string(); 
 
    println!("Enter student age:"); 
    let mut age = String::new(); 
    io::stdin().read_line(&mut age).expect("Failed to read line"); 
    let age: u32 = age.trim().parse().expect("Invalid age"); 
 
    println!("Enter student grade:"); 
    let mut grade = String::new(); 
    io::stdin().read_line(&mut grade).expect("Failed to read line"); 
    let grade: u32 = grade.trim().parse().expect("Invalid grade"); 
 
    Student { name, age, grade }
}
