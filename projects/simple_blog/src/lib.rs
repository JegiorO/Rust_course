use std::io;
use std::collections::HashMap;

pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16
}

pub struct Post {
    pub header: String,
    pub author: String,
    pub date: Date,
    pub posttext: String,
}

pub struct Blog {
    pub last_key: u64,
    pub posts: HashMap<u64, Post>
}

impl Date {
    //struct constructor with simple check for correct values
    pub fn new(day: u8, month: u8, year: u16) -> Result<Date, io::Error>{
        if day < 1 || day > 31 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Неверное значение дня"));
        }
        if month < 1 || month > 12 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Неверное значение месяца"));
        }
        Ok(Date{
            day: day,
            month: month,
            year: year
    })
    }
}

impl Post {
    //fields getters
    fn get_header(&self) -> &str {
        &self.header
    }

    fn get_author(&self) -> &str {
        &self.author
    }

    fn get_posttext(&self) -> &str {
        &self.posttext
    }

    fn get_date(&self) -> &Date {
        &self.date
    }

    //printing fields in terminal
    pub fn print_info(&self) {
        println!("{}", self.get_header());
        println!("by {}", self.get_author());
        println!("{}", self.get_posttext());
        println!("Posted on {}/{}/{}", self.get_date().day, self.get_date().month, self.get_date().year);
        println!("");
    }

    //constructor
    pub fn create(&self, d: Date, h: String, a: String, pt: String) -> Post{
        Post{
            header: h,
            author: a,
            date: d,
            posttext: pt,
        }
    }
}

impl Blog {
    //adding post to hashmap and defining its id (key)
    pub fn add_post(&mut self, p: Post) {
        let id = self.last_key + 1;
        self.last_key += 1;
        self.posts.insert(id, p);
        println!("post added successfully with id of {}", id)
    }

    //getting hashmap with posts
    pub fn get_posts(&self) -> HashMap<u64, &Post> {
        let mut post_map: HashMap<u64, &Post> = HashMap::new();
        
        for (u, (k, p)) in self.posts.iter().enumerate() {
            let i = k;
            post_map.insert(*k, p);
        }

        post_map
    }

    //printing posts in terminal
    pub fn show_posts(&self) {
        for (k ,p) in &self.get_posts() {
            p.print_info();
            println!("id: {}", k);
        }
    }

    //getting post from hashmap by id
    pub fn find_by_id(&self, id_of_searched:u64) -> Result<&Post, String>{
        match self.posts.get(&id_of_searched) {
            Some(post) => Ok(post),
            None => Err(String::from("Post not found"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //creating data for post for tests
    fn create_test_date() -> Date {
        Date{
            day: 1,
            month: 2,
            year: 2003
        }
    }
    
    //creating test post
    fn create_test_post() -> Post {
        let post = Post {
            header: String::from("HEADER"),
            author: String::from("JegiorO"),
            date: create_test_date(),
            posttext: String::from("this is post"),
        };
        post
    }

    #[test]
    //checking that post we added is shown correctly
    fn correct_post_adding() {
        let mut b = Blog {
            last_key: 0,
            posts: HashMap::new()
        };

        let p = create_test_post();
        b.add_post(p);

        let a = match b.get_posts().get(&1) {
            Some(p) => p.get_author(),
            None => "NULL"
        };

        let t = match b.get_posts().get(&1) {
            Some(p) => p.get_posttext(),
            None => "NULL"
        };
        let h = match b.get_posts().get(&1) {
            Some(p) => p.get_header(),
            None => "NULL"
        };
        let d = match b.get_posts().get(&1) {
            Some(p) => p.get_date(),
            None => &Date { day: 0, month: 0, year: 0 }
        };

        assert_eq!(a, "JegiorO");
        assert_eq!(h, "HEADER");
        assert_eq!(t, "this is post");
        assert_eq!(d.day, 1);
        assert_eq!(d.month, 2);
        assert_eq!(d.year, 2003);
    }

    #[test]
    //checking that we can get added post by its id
    fn correct_getter_by_id() {
        let mut b = Blog {
            last_key: 0,
            posts: HashMap::new()
        };

        let p = create_test_post();
        b.add_post(p);

        let p = match b.find_by_id(1) {
            Ok(p) => p,
            Err(e) => panic!("not found")
        };

        assert_eq!(p.get_author(), "JegiorO")
    }

    #[test]
    //checking that we handle situation of searching post that doesnt exist
    fn correct_error_handling() {
        let mut b = Blog {
            last_key: 0,
            posts: HashMap::new()
        };

        let p = create_test_post();
        b.add_post(p);

        let p = match b.find_by_id(2) {
            Ok(p) => String::from("found"),
            Err(e) => e
        };

        assert_eq!(p, "Post not found")
    }

    #[test]
    fn all_posts_are_shown() {
        let mut b = Blog {
            last_key: 0,
            posts: HashMap::new()
        };

        let p = create_test_post();
        b.add_post(p);

        let a = match b.get_posts().get(&1) {
            Some(p) => p.get_author(),
            None => "NULL"
        };

        let t = match b.get_posts().get(&1) {
            Some(p) => p.get_posttext(),
            None => "NULL"
        };
        let h = match b.get_posts().get(&1) {
            Some(p) => p.get_header(),
            None => "NULL"
        };
        let d = match b.get_posts().get(&1) {
            Some(p) => p.get_date(),
            None => &Date { day: 0, month: 0, year: 0 }
        };

        assert_eq!(a, "JegiorO");
        assert_eq!(h, "HEADER");
        assert_eq!(t, "this is post");
        assert_eq!(d.day, 1);
        assert_eq!(d.month, 2);
        assert_eq!(d.year, 2003);

        let d = create_test_post();
        b.add_post(d);

        let a = match b.get_posts().get(&2) {
            Some(p) => p.get_author(),
            None => "NULL"
        };

        let t = match b.get_posts().get(&2) {
            Some(p) => p.get_posttext(),
            None => "NULL"
        };
        let h = match b.get_posts().get(&2) {
            Some(p) => p.get_header(),
            None => "NULL"
        };
        let d = match b.get_posts().get(&2) {
            Some(p) => p.get_date(),
            None => &Date { day: 0, month: 0, year: 0 }
        };

        assert_eq!(a, "JegiorO");
        assert_eq!(h, "HEADER");
        assert_eq!(t, "this is post");
        assert_eq!(d.day, 1);
        assert_eq!(d.month, 2);
        assert_eq!(d.year, 2003);
    }
}