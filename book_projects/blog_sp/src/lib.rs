pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}

pub struct PendingReviewPost {
    content: String
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Post;

    #[test]
    fn approve_post() {
        let mut post = Post::new();
        let text = "I ate a Salad for lunch today";
        post.add_text(text.clone());

        let post = post.request_review();

        let post = post.approve();

        assert_eq!(text, post.content());
    }
}