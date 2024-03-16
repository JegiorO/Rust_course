pub struct Post {
    state: Option<Box<dyn State>>, //draft, pendingReview or published - three possible states of post
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            //when post is created it always is a draft
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    //func for adding text to the post's content
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    //
    pub fn content(&self) -> &str {
        ""
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

//trait to change state of post
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

//implementing state trait
impl State for Draft {
    //when the post is reviewed, state is changed from draft to PendingReview
    fn request_review(self: Box<Self>) -> Box<dyn State> { //notice that can only be used with box
        Box::new(PendingReview {})
    }

    //same when post is approved, state will be changed
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}
//again, need to add realization fot all states
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
