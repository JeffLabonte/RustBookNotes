/// This is the first example
/// It is using the State pattern
//pub struct Post {
//    state: Option<Box<dyn State>>, // dyn Statement makes it possible to change structs dynamically with other struct implement with the State trait
//    content: String,
//}
//
//impl Post {
//    pub fn new() -> Post {
//        Post {
//            state: Some(Box::new(Draft {})),
//            content: String::new(),
//        }
//    }
//
//    pub fn add_text(&mut self, text: &str) {
//        self.content.push_str(text);
//    }
//
//    pub fn content(&self) -> &str{
//        self.state.as_ref().unwrap().content(&self)
//    }
//
//    pub fn request_review(&mut self){
//        if let Some(s) = self.state.take() {
//            self.state = Some(s.request_review())
//        }
//    }
//
//    pub fn approve(&mut self){
//        if let Some(s) = self.state.take() {
//            self.state = Some(s.approve())
//        }
//    }
//}
//
//trait State {
//    fn request_review(self: Box<Self>) -> Box<dyn State>; // Self reprensent the Type ( the struct )
//    fn approve(self: Box<Self>) -> Box<dyn State>;
//    fn content<'a>(&self, _post: &'a Post) -> &'a str {
//        ""
//    }
//}
//
//struct Draft {}
//
//impl State for Draft {
//    fn request_review(self: Box<Self>) -> Box<dyn State>{
//        Box::new(PendingReview {})
//    }
//
//    fn approve(self:Box<Self>) -> Box<dyn State>{
//        self
//    }
//}
//
//struct PendingReview {}
//
//impl State for PendingReview{
//    fn request_review(self: Box<Self>) -> Box<dyn State>{
//        self
//    }
//
//    fn approve(self: Box<Self>) -> Box<dyn State>{
//        Box::new(Published {})
//    }
//}
//
//struct Published {}
//
//impl State for Published {
//    fn request_review(self: Box<Self>) -> Box<dyn State>{
//        self
//    }
//
//    fn approve(self: Box<Self>) -> Box<dyn State>{
//        self
//    }
//
//    fn content<'a>(&self, post: &'a Post) -> &'a str{
//        &post.content
//    }
//}

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost{
        DraftPost{
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str{
        &self.content
    }
}

impl DraftPost{
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
