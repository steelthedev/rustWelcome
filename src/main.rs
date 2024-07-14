

fn main() {

    let user: User = build_new_user(
        String::from("akinwumikaliyanu@gmail.com"), 
        String::from("steelthedev"), 
        true,
        5);

    let mut book: Book = build_new_book(
        String::from("Struct in rust"),
        user,
        56,
        true);

    book.print_book_name();
    book.alter_book_name(String::from("Altered Carbon"));    
    book.print_author_details();
}


struct Book{
    title: String,
    author: User,
    pages: u32,
    available: bool,
}

#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String, 
    sign_in_count: u64,
}


fn build_new_user(email: String, username: String, active: bool, sign_in_count: u64) -> User{
    User { active: active, username: username, email: email, sign_in_count: sign_in_count }
}

fn build_new_book(title: String, author: User, pages:u32, available: bool) -> Book{
    Book { title: title, author: author, pages: pages, available: available }
}


impl Book {
    fn print_book_name(&self){
        println!("Book name is {}", self.title)
    }

    fn alter_book_name(&mut self, new_book_name:String){
        self.title = new_book_name;
        self.print_book_name()
    }

    fn print_author_details(&self){
        println!("Author's details, {:?}", self.author.print_user_details());
    }
}

impl User{
    fn print_user_details(&self){
        println!("{:?}", self);
    }
}