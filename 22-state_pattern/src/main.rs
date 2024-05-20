use state_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_content("Here is my first post.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    let content = post.content();
    assert_eq!("Here is my first post.", content);
    println!("Post content: {}", content);
}
