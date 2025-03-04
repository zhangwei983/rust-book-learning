use futures::executor::block_on;

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "Vincent".to_string(),
        name: String::from("My song"),
    }
}

async fn sing_song(song: Song) {
    println!( "Singing the song '{}' from {}.", song.name, song.author);
}

async fn dance() {
    println!("Dancing");
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // Use `.await` instead of `block_on` to prevent blocking the thread, as it's possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
