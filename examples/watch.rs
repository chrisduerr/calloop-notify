use std::path::Path;

use calloop::EventLoop;
use calloop_notify::NotifySource;
use calloop_notify::notify::{RecursiveMode, Watcher};

fn main() {
    // Create calloop event loop.
    let mut event_loop = EventLoop::try_new().unwrap();
    let loop_handle = event_loop.handle();

    // Watch current directory recursively.
    let mut notify_source = NotifySource::new().unwrap();
    notify_source.watch(Path::new("."), RecursiveMode::Recursive).unwrap();

    // Insert notify source into calloop.
    loop_handle
        .insert_source(notify_source, |event, _, _| {
            println!("Notify Event: {event:?}");
        })
        .unwrap();

    // Dispatch event loop.
    loop {
        event_loop.dispatch(None, &mut ()).unwrap();
    }
}
