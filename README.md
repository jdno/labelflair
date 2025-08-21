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

# Create a group of labels that is named "categories"
[labels.categories]
prefix = "C-"
colors = { tailwind = "red" }
labels = ["bug", "feature"]
```

Running `labelflair` in the same directory as the configuration file will
generate a `labels.yml` file that can be used with GitHub Actions such as
[EndBug/label-sync](https://github.com/EndBug/label-sync) to create the labels
on GitHub.

### Configuration

The configuration file is a TOML file that defines the labels to be generated.
Labels are grouped, and each group can have a prefix, a color generator, and a
list of labels.

```toml
[labels.<group_name>]
```

At a minimum, each group must choose a color generator and define a set of
labels.

```toml
[labels.minimal]
colors = { tailwind = "blue" }
labels = ["bug", "feature", "enhancement"]
```

You can also specify a prefix for the labels in the group:

```toml
[labels.area]
prefix = "A-"
colors = { tailwind = "green" }
labels = ["backend", "frontend"]
```

Labels can either be defined as a simple string or as an object with a `name`
and `description`:

```toml
[labels.changelog]
colors = { tailwind = "slate" }
labels = [
    { name = "major", description = "Major changes" },
    { name = "minor", description = "Minor changes" },
    { name = "patch", description = "Patch changes" },
]
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
