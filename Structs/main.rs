// Structs

struct Book {
    title: String,
    author: String, 
    pages: u32,
    available: bool,
}

struct User {
    active: bool,
    username: String, 
    email: String, 
}

// struct methods

impl Book {

    fn new(title: String, author: String, pages: u32, available: bool) -> Self {
        Self{ title, author, pages, available }
    }

    fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    fn print_book(&self) {
        println!("Title: {}, Author: {}, Pages: {}, Avaiable: {}", self.title, self.author, self.pages, self.available);
    }
}



fn main(){

    let mut user1: User = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someusername@m.com"),
    };

    user1.email = String::from("email@gmail.com");
    println!("Email: {}", user1.email);

    // Return a struct from a function

    fn build_user(email: String, username: String) -> User{
        User{
            active: true,
            username: username,
            email: email,
        }
    }

    let user2: User = build_user(String::from("email@gmail.com"), String::from("username"));
    println!("user2: {}, {}, {}", user2.active, user2.username, user2.email);

    // create instances from another instance

    let user3: User = User {
        email: String::from("another@gmail.com"),
        ..user2
    };
    println!("user3: {}, {}, {}", user3.active, user3.username, user3.email);

    // using methods

    let mut book:Book = Book::new("The Alchemist".to_string(), "Paulo Cohlo".to_string(), 200, true);
    
    book.print_book();
    book.set_title(String::from("Only Alchemist"));
    book.print_book();

}