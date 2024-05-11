mod thread_spawn;
mod message_passing;

fn main() {
    thread_spawn::test_thread_spawn();
    println!("");

    message_passing::test_message_passing();
}
