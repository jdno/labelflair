# 🌈 Labelflair

_Generate a colorful palette of labels for your GitHub Issues._

Labelflair is a simple tool to generate colorful labels for GitHub Issues from a
[configuration] file. It makes it easy to define groups of related labels,
assign similar colors to them, and then synchronize them with GitHub using a
[GitHub Action].

- [Usage](#usage)
  - [Command-line Application](#command-line-application)
  - [GitHub Action](#github-action)
- [Configuration](#configuration)
  - [Label Groups](#label-groups)
  - [Individual Labels](#individual-labels)
- [How-To Guides](#how-to-guides)
  - [Rename a Label](#rename-a-label)
- [Development](#development)

## Usage

Labelflair can be installed as a command-line application and used locally to
generate labels, or it can be used as a [GitHub Action] to automatically create
and update labels in your GitHub repository.

### Command-line Application

The `labelflair` command-line application reads a [configuration] file and uses
that to generate a set of labels for GitHub Issues. It can be installed using
`cargo`:

```bash
cargo install labelflair-cli
```

Once installed, you can run `labelflair` in the directory where your
[configuration] file is located. It will generate a `labels.yml` file that can
be used with GitHub Actions such as [EndBug/label-sync] to create the labels
on GitHub.

### GitHub Action

The easiest way to use Labelflair is as a GitHub Action. Simply create a
[configuration] file in your repository and then add the following step to your
GitHub Actions workflow:

```yaml
- name: "Sync GitHub Issues labels"
  uses: "jdno/labelflair@v0.2.0"
  with:
    config-file: ".github/labelflair.toml"
    # Only run this step on the main branch to avoid creating labels in pull requests
    dry-run: ${{ github.ref != 'refs/heads/main' }}
```

This will generate the labels from your `labelflair.toml` configuration file,
and run another GitHub Action to synchronize them with your repository.

## Configuration

The configuration file is a TOML file that defines the labels to be generated.
It provides two different ways to define labels: as part of a group of related
labels, or as individual labels.

### Label Groups

Labels that are related to each other (e.g. categories, priorities, or statuses)
can be grouped together. In a group, the labels share a common color scheme and
optionally a common prefix.

```toml
[[group]]
prefix = "C-"
colors = { tailwind = "red" }
labels = ["bug", "feature"]
```

At a minimum, each group must choose a color generator and define a set of
labels.

```toml
[[group]]
colors = { tailwind = "blue" }
labels = ["bug", "feature", "enhancement"]
```

### `colors`

The `colors` property defines how the colors for the labels in this group are
generated. The following color generators are available:

#### Fixed Color

The `fixed` color generator allows you to specify a single color that will be
used for all labels in the group. The color must be specified as a hex color
code:

```toml
colors = { fixed = "#4ade80" }
```

#### Tailwind CSS

The `tailwind` color generator generates colors based on the
[Tailwind CSS color palette][tailwind]. You can choose any of the available
colors from Tailwind CSS, which you can find in the Tailwind CSS documentation.
Not all shades for each color are used to ensure a good contrast between the
different colors.

```toml
colors = { tailwind = "slate" }
```

If you specify more labels than there are shades, the color generator will cycle
through the available shades again.

### `labels`

Labels can either be defined as a simple string or as an object with a `name`
and `description`.

This is a list of simple labels:

```toml
labels = ["bug", "feature", "enhancement"]
```

But you can also provide a description for each label:

```toml
labels = [
    { name = "major", description = "Major changes" },
    { name = "minor", description = "Minor changes" },
    { name = "patch", description = "Patch changes" },
]
```

The two forms can be mixed in the same list:

```toml
labels = [
    "good first issue",
    { name = "help wanted", description = "We need your help!" },
]
```

### Individual Labels

Labels can also be defined individually, outside a group. This is useful for
one-off labels that don't have any related labels. Individual labels support the
same properties as labels in a group, but they must also specify a color.

```toml
[[label]]
name = "good first issue"
description = "Good first issue for newcomers"
color = "#4ade80"
aliases = ["help wanted"]
```

## How-To Guides

This section provides a few examples of common tasks you might want to perform
with Labelflair.

### Rename a Label

Labels can be renamed by changing their name in the configuration file and
adding the old name as an alias. For example, if you want to rename the label
`bug` to `defect`, you can change the configuration from this:

```toml
[[group]]
prefix = "C-"
colors = { tailwind = "red" }
labels = ["bug"]
```

to this:

```toml
[[group]]
prefix = "C-"
colors = { tailwind = "red" }
labels = [{ name = "defect", aliases = ["bug"] }]
```

This allows Labelflair's [GitHub Action] to detect that the label has been
renamed and update it accordingly on GitHub.

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

[configuration]: #configuration
[EndBug/label-sync]: https://github.com/EndBug/label-sync
[flox]: https://flox.dev
[github action]: https://github.com/marketplace/actions/labelflair
[tailwind]: https://tailwindcss.com/docs/colors
