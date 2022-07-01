use step_2_1::{Deleted, New, NewPost, Post};

fn main() {
    let post: Post<New> = NewPost {
        id: 1u64.into(),
        user_id: 42u64.into(),
        title: "Interesting title".to_string().into(),
        body: "Post body".to_string().into(),
    }
    .into();

    // Compile errors:
    // post.allow();
    // post.deny();
    // post.delete();

    let unmoderated = post.publish();

    // Compile errors:
    // post.publish();
    // post.deny();
    // post.allow();
    // post_unmoderated.delete();
    // post_unmoderated.delete();

    let post = unmoderated.allow();

    // Compile errors:
    // post.publish();
    // post.allow();
    // post.deny();
    // unmoderated.publish();
    // unmoderated.allow();
    // unmoderated.deny();
    // unmoderated.delete();

    let _deleted = post.delete();

    let post: Post<New> = NewPost {
        id: 2.into(),
        user_id: 42.into(),
        title: "Bad title".to_string().into(),
        body: "Bad post".to_string().into(),
    }
    .into();

    let _denied: Post<Deleted> = post.publish().deny();
}
