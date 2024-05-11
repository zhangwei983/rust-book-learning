mod thread_spawn;
mod message_passing;
mod shared_state;
mod thread_local;

fn main() {
    thread_spawn::test_thread_spawn();
    println!("");
    message_passing::test_message_passing();
    println!("");
    shared_state::test_shared_state();
    println!("");
    thread_local::test_thread_local();
}
