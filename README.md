<div align="center">

# Env Var Helpers

[![Crates.io](https://img.shields.io/crates/v/env_var_helpers)](https://crates.io/crates/env_var_helpers)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue)](#license)
[![Downloads](https://img.shields.io/crates/d/env_var_helpers)](https://crates.io/crates/env_var_helpers)
[![Docs](https://docs.rs/env_var_helpers/badge.svg)](https://docs.rs/env_var_helpers/latest/env_var_helpers)
[![Twitch Status](https://img.shields.io/twitch/status/coruscateor)](https://www.twitch.tv/coruscateor)

[X](https://twitter.com/Coruscateor) | 
[Twitch](https://www.twitch.tv/coruscateor) | 
[Youtube](https://www.youtube.com/@coruscateor) | 
[Mastodon](https://mastodon.social/@Coruscateor) | 
[GitHub](https://github.com/coruscateor) | 
[GitHub Sponsors](https://github.com/sponsors/coruscateor)

Macros and functions that help provide access to environment variable values.

<br/>

</div>

<br/>

```rust

use env_var_helpers::cargo::crates::*;

println!("{}\n", cargo_env());

println!("{}\n", cargo_pkg_version_env());

println!("{}\n", cargo_pkg_name_env());

println!("{}\n", cargo_pkg_readme_env());

```

<br/>

## Compiler:

Build with the latest stable compiler.

<br/>

## Todo:

- Add more documentation
- Add more functions

<br/>

## Coding Style

This project uses a coding style the emphasises the use of white space over keeping the line and column counts as low as possible.

So this:

```rust
fn bar() {}

fn foo()
{

    bar();

}

```

Not this:

```rust
fn bar() {}

fn foo()
{
    bar();
}

```

<br/>

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0 (see also: https://www.tldrlegal.com/license/apache-license-2-0-apache-2-0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT (see also: https://www.tldrlegal.com/license/mit-license))

at your discretion

<br/>

## Contributing

Please clone the repository and create an issue explaining what feature or features you'd like to add or bug or bugs you'd like to fix and perhaps how you intend to implement these additions or fixes. Try to include details though it doesn't need to be exhaustive and we'll take it from there (dependant on availability).

<br/>

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
