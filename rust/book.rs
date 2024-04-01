struct Book {
    title: String,
    author: String,
    isbn: String,
    year: u32,
}

impl Book {
    fn new(title:String, author:String, isbn:String, year:u32)-> Self{
        Book {
            title,
            author,
            isbn,
            year,
        }
    }
    
    fn print_details(&self){
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("ISBN: {}", self.isbn);
        println!("Publication Year: {}", self.year);
    }
}

fn main() {
    let book1 = Book::new(
        "Rakesh Reddy".to_string(),
        "Roczy".to_string(),
        "914-489-3124".to_string(),
        2018,
    );
    
    println!("The details of Book:");
    book1.print_details();
}
