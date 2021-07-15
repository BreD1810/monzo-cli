Monzo CLI
===

A small commandline tool for getting information about your Monzo account, written in Rust.

Installation
---
Ensure you have cargo installed, and run
```
cargo install --path .
```

Usage
---
To see available commands, use:
```
monzo --help
```

To link your account, register a client on the [Monzo developer portal](https://developers.monzo.com/).
Next, run `monzo auth`, providing the Client ID and Client Secret.
Navigate to the link provided, and follow the authorisation steps provided by Monzo.
Finally, be sure to allow access when requested by your Monzo app.

