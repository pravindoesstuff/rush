1. `unwrap()` - Use it conservatively. In almost all cases, using a `match` or `if let Err(e)` to display errors is preferred over the program possible panicking with a backtrace
2. Before you submit a PR, make sure you `cargo fmt` and resolve **any and all** messages from `cargo clippy`
3. Windows support holds no priority, but small changes are welcome. Large architectural changes will take some convincing (however I don't envision this being a problem)
4. Don't use variable names like `foo` or `var`. Try to be somewhat specific 
6. These probably won't ever end up being read by the eyes of another person and are basically just for me to hold myself 'somewhat' accountable
