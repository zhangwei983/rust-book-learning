mod thread_spawn;
mod message_passing;
mod shared_state;

fn main() {
    thread_spawn::test_thread_spawn();
    println!("");
    //message_passing::test_message_passing();
    println!("");
    shared_state::test_shared_state();
}
