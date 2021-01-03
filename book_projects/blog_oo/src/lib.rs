pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content = self.state.as_ref().unwrap().add_text(&self.content, text)
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    pub fn reject(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.reject())
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn add_text(&self, content: &String, text: &str) -> String {
        content.to_owned()
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approve_counter: 1 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(&self, content: &String, text: &str) -> String {
        content.to_owned() + text
    }
}

struct PendingReview {
    approve_counter: i32
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.approve_counter < 2 {
            return Box::new(PendingReview { approve_counter: self.approve_counter + 1 });
        }
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approve_post() {
        let mut post = Post::new();
        let text = String::from("I ate a salad for lunch today");

        post.add_text(text.as_str());
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        post.approve();
        assert_eq!(text, post.content());

        post.add_text("I do not can change after review, sorry");
        assert_eq!(text, post.content())
    }

    #[test]
    fn reject_post() {
        let mut post = Post::new();
        let text = String::from("I ate a salad for lunch today");

        post.add_text(text.as_str());
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.reject();
        assert_eq!("", post.content());
    }
}