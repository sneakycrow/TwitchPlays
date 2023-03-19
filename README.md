# TwitchPlays

Originally based on [DougDougGithub/TwitchPlays][original-repo], this repository aims to take those scripts and set
a GUI in front of it, making the user experience a lot friendlier. In addition, it aims to rewrite the original scripts
as to not require the Python runtime.

[original-repo]:https://github.com/DougDougGithub/TwitchPlays

## Development

### Requirements

- Node (version in `.nvmrc`)
- rustc v1.68.0
- Tauri CLI

_You can install the Tauri CLI via `cargo install tauri-cli`_

To run the app in development mode run:

`cargo tauri dev`

That's it. :) 