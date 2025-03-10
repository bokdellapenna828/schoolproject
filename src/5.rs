use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        // do something in this thread...
    });

    handle.join();
}
