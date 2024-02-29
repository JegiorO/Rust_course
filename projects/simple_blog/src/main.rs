use std::collections::HashMap;
use std::io;
use simple_blog::Post;
use simple_blog::Date;
use simple_blog::Blog;

fn main() {
    //initializing blog
    let mut my_blog: Blog = Blog {
        last_key: 0,
        posts: HashMap::new()
    };

    //starting workflow
    blog_cycle(&mut my_blog);
}

//func to handle user input
fn read_line() -> String {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read choice");
    input_text
}

fn blog_cycle(blog: &mut Blog) {
    loop {
    println!("");
    println!("What would you like to do? (Enter a number)");
    println!("1. Add new post");
    println!("2. Scroll the post wall");
    println!("3. Find a post by id");
    println!(">");

    let mut input_text = String::new();
    let choice;
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read choice");
    let trimmed = input_text.trim();
            match trimmed.parse::<u8>() {
                Ok(i) => choice = i,
                Err(..) => {
                    println!("this was not an integer: {}", trimmed);
                    continue;
                }
                };

    match choice {
        1 => {
            let mut input_text = String::new();
            let day: u8;
            let month: u8;
            let year: u16;
            println!("Type date of post adding");

            println!("day: ");
            io::stdin()
                .read_line(&mut input_text)
                .expect("Failed to read line");
            let trimmed = input_text.trim();
            match trimmed.parse::<u8>() {
                Ok(i) => day = i,
                Err(..) => {
                    println!("this was not an integer: {}", trimmed);
                    continue;
                }
                };
            
            let mut input_text = String::new();
            println!("month: ");
            io::stdin()
                .read_line(&mut input_text)
                .expect("Failed to read line");
            let trimmed = input_text.trim();
            match trimmed.parse::<u8>() {
                Ok(i) => month = i,
                Err(..) => {
                    println!("this was not an integer: {}", trimmed);
                    continue;
                }
                };
            
            let mut input_text = String::new();
            println!("year: ");
            io::stdin()
                .read_line(&mut input_text)
                .expect("Failed to read line");
            let trimmed = input_text.trim();
            match trimmed.parse::<u16>() {
                Ok(i) => year = i,
                Err(..) => {
                    println!("this was not an integer: {}", trimmed);
                    continue;
                }
                };

            let current_date = match Date::new(day, month, year) {
                Ok(date) => date,
                Err(..) => {
                    println!("incorrect date!");
                    continue
                }
            };

            let mut _author: String;
            println!("Author: ");
            _author = read_line();
            _author.pop();

            let mut _header: String;
            println!("Header: ");
            _header = read_line();
            _header.pop();

            let mut _text: String;
            println!("Text: ");
            _text = read_line();
            _text.pop();

            let post = Post {
                header: _header,
                author: _author,
                date: current_date,
                posttext: _text,
            };
            blog.add_post(post);
        }
        2 => {
            blog.show_posts();
        }
        3 => {
            let mut input_text = String::new();
            let id;
            println!("id: ");
            io::stdin()
                .read_line(&mut input_text)
                .expect("Failed to read line");
            let trimmed = input_text.trim();
            match trimmed.parse::<u64>() {
                Ok(i) => id = i,
                Err(..) => {
                    println!("this was not an integer: {}", trimmed);
                    continue;
                }
                };

            match blog.find_by_id(id) {
                Ok(post) => post.print_info(),
                Err(e) => println!("{}", e)
            }
        }
        _ => {
            println!("incorrect num");
            continue;
        }
    }
}
}