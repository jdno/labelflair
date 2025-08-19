# 🌈 Labelflair

Generate a colorful palette of labels for your GitHub Issues.

`labelfair` is a command-line application that takes a simple configuration file
and generates a set of colorful labels for GitHub Issues.

## Usage

The `labelflair` command-line application reads a configuration file and uses
that to generate a set of labels for GitHub Issues. The configuration file looks
like this:

```toml
# .github/labelflair.toml
version = "1"

[labels.categories]
prefix = "C-"
colors = { tailwind = "red" }
labels = ["bug", "feature"]
```

## Development

Contributing to Labelflair is easy! We use [Flox] to make it easy to set up a
development environment with all the required tools, so head over to its website
to learn how to install it on your machine. Once you're done, clone this
repository and run the following command to set up the development environment:

```bash
flox activate
```

This will install all the dependencies and activate them in a new shell for you.
When you're done working on Labelflair, simple exit out of the shell with `exit`
or `Ctrl+D` to deactivate the environment.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT)
  or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[flox]: https://flox.dev
