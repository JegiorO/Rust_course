use std::io;

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
    pub id: u64
}

pub struct Blog {
    pub posts: Vec<Post>
}

impl Date {
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
    pub fn print_info(&self) {
        println!("{}", self.header);
        println!("by {}", self.author);
        println!("{}", self.posttext);
        println!("Posted on {}/{}/{}", self.date.day, self.date.month, self.date.year);
        println!("post id: {}", self.id);
        println!("");
    }

    pub fn create(&self, d: Date, h: String, a: String, pt: String, _id: u64) -> Post{
        Post{
            header: h,
            author: a,
            date: d,
            posttext: pt,
            id: _id
        }
    }
}

impl Blog {
    pub fn add_post(&mut self, p: &mut Post) {
        p.id = match self.posts.last() {
            Some(v) => v.id+1,
            None => 1
        };
        let copy = Post{
            header: p.header.clone(),
            author: p.author.clone(),
            date: Date{day: p.date.day, month: p.date.month, year: p.date.year},
            posttext: p.posttext.clone(),
            id: p.id
        };

        self.posts.push(copy);
        println!("post added successfully with id of {}", p.id)
    }

    pub fn show_posts(&self) -> Vec<u64> {
        let mut v = vec![];
        for i in &self.posts {
            i.print_info();
            v.push(i.id)
        }
        v
    }

    pub fn find_by_id(&self, id_of_searched:u64) -> Result<&Post, String>{
        for p in &self.posts {
            if p.id == id_of_searched {
                return Ok(p);
            }
        }
        Err(String::from("Post not found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_post() {
        let _date = Date::new(1, 2, 2001);
        let mut bl = Blog {
            posts: Vec::new()
        };
        let mut p1 = &mut Post{
            header: String::from("HHH"),
            author: String::from("AFFOR ZHHET"),
            date: Date{
                day: 1,
                month: 1,
                year: 2001
            },
            posttext: String::from("zdravia vsem"),
            id: 1
        };

        bl.add_post(p1);
        
        //println!(bl.show_posts().contains(&1));
        assert!(bl.show_posts().contains(&1));

    }

    #[test]
    fn good_overcome_of_error_id_not_found() {
        let mut bl = Blog {
            posts: Vec::new()
        };
        let mut p1 = &mut Post{
            header: String::from("HHH"),
            author: String::from("AFFOR ZHHET"),
            date: Date{
                day: 1,
                month: 1,
                year: 2001
            },
            posttext: String::from("zdravia vsem"),
            id: 1
        };

        let va = match bl.find_by_id(2) {
            Ok(_) => String::new(),
            Err(s) => s
        };

        assert!(va.eq(&String::from("Post not found")));
    }
}
