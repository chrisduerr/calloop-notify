# calloop-notify

[Calloop] adapter for [Notify].

This crate provides an `EventSource` implementation for Notify, allowing easy
integration into the Calloop event source. This makes it possible to easily
watch multiple files in a non-blocking fashion, using the native operating
system APIs.

[Calloop]: https://github.com/Smithay/calloop
[notify]: https://github.com/notify-rs/notify

## Example

```rust
use std::path::Path;

use calloop::EventLoop;
use calloop_notify::NotifySource;
use notify::{RecursiveMode, Watcher};

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
```
