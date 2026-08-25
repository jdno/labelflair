# Labelflair

Labelflair generates a colorful palette of labels for GitHub Issues. The
`labelflair` crate turns a TOML configuration into a list of labels, the
`labelflair-cli` crate wraps that library in a command-line application, and
`action.yml` publishes both as a GitHub Action that synchronizes the labels
with a repository.

## Language

- Use American English spelling, e.g. "color" not "colour".

## Markdown

- Use title case in headings and titles.
- Always use the Oxford comma.
- Use reference-style Markdown links, not inline links.
- Table cells must be single-line. Markdown does not support multi-line cells;
  each newline starts a new row. Ignore line length limits for table rows.

## Rust

### Dependencies

- All versions managed in root `Cargo.toml`, crates import from workspace.
- Write dependency entries without comments. Do not describe what a package
  does, and do not explain a version requirement. Reasoning that matters, such
  as why a floor cannot go lower, belongs in the commit message.

### Derives

- Standard trait order: Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
  Debug, Default
- Third-party derives: alphabetical by crate, then by macro.
- First list traits from the standard library, then from external crates.

### Documentation

- Both crates warn on missing documentation, for public and for private items.
  Every module, type, field, and function needs a doc comment.
- Documentation should explain the "why", not just the "what".
  - **Types**: Explain design decisions, invariants, and relationships to other
    types.
  - **Functions**: Document side effects, caller considerations, and non-obvious
    behavior.
  - **Modules**: Explain the module's role in the system and key concepts.
- Write documentation for a reader that has no prior context, and especially no
  knowledge of the conversation that led to the creation of the code.
- Write for a consumer of the published crate. Internal rationale, such as
  which library a function hides, stays out of the documentation. Document what
  the API does, what it requires of the caller, and how it fails.
- Write function/method docs in third-person singular
  ("Returns the..." not "Return the...").
- Do not add a trailing period on the title (i.e. the first line).
- Use reference-style links in doc comments, not inline path links. Paths like
  `super::` and `crate::` should not appear in rendered documentation.
- Use the `/simple-english::simple-english` skill to adhere to the ASD-STE100
  standard for Simplified Technical English.

### Modules

- One public type per module, use submodules for related types.
- A module that has submodules lives in `<name>.rs` beside a `<name>/`
  directory. It declares its submodules privately and re-exports their public
  types with `pub use self::<submodule>::<Type>`.
- Prefer `pub` over `pub(crate)`. Visibility should come from module
  structure, not access modifiers. If a type needs restricted visibility,
  that is usually a signal to restructure the modules.

### Tests

- Use blank lines to separate Arrange/Act/Assert phases.
- Test functions ordered alphabetically within modules.
- Name tests descriptively: `function_name_<condition>_<result>`, e.g.
  `greet_with_name_returns_greeting`.
- Do not test compiler-derived traits (Eq, Ord, Hash, Clone, etc.). Only test
  auto traits (Send, Sync, Unpin) and custom behavior like builder round-trips.
- Each test should have exactly one assertion.
- The command-line application is tested with [trycmd][trycmd]. Each case is a
  directory under `crates/labelflair-cli/tests/commands/`.

### Errors

- Define one error enum per fallible action, named after the action and its
  object (e.g. `LoadConfigError`, `GenerateLabelsError`), never after the
  component that raises it.
- Use struct variants. The underlying cause is a field named `source`, and
  context the message needs is carried in named fields. A variant only
  carries context its own layer knows.
- Variants name the failure condition together with its object (e.g.
  `UnknownColor`, `MissingLabels`, `UnreadableConfig`), never the step that
  failed. Name the condition at the certainty you have.
- A variant with a `source` reads "failed to ..." in its message; a variant
  that is itself the diagnosis states its condition declaratively.
- A failure an operation reports is a value (`Failure`), not an error. It
  travels in a regular field, never as a `source`.
- Start `.context()` and similar error messages with a lowercase letter
  (e.g., `"failed to read the configuration file"`). Error messages may be
  embedded in larger error chains, and Rust convention is lowercase for these
  fragments.

### Type System

- Primitives (`i64`, `String`, `bool`) are only allowed at system boundaries.
  Owned structs must always define a newtype (e.g. with `typed-fields`).
- Use enums with meaningful variants instead of bool parameters.
- Fields must never be `pub`. Implement getters instead, e.g. with `getset`.

## Version Control

- Never commit directly to `main`, always create a branch or worktree.
- Every commit should be a logical unit of change.
- Every commit must build and pass all checks. Use `just` recipes for
  verification (e.g. `just pre-commit`).
- Fixes and refactoring should be in separate commits from features.
- Each pull request should have one primary commit with a well-crafted
  message — this is what lands in the Git history. Follow-up fixups within
  the same PR can use simple one-liner messages since they get squashed into
  the primary commit on merge.
- Reuse the commit message as the pull request description, but reflow each
  paragraph onto one line, because GitHub renders every newline as a line
  break. Do not use `gh pr create --fill`.
- `CHANGELOG.md` follows [Keep a Changelog][keep-a-changelog]. It is written
  when a release is prepared, in its own pull request. A pull request that
  adds a feature or fixes a bug does not add an entry.

### Commit Messages

- We use Git as our Version Control System and GitHub to host the code.
- We use pre-commit hooks to verify the changes before committing them.
- We follow this [style guide][git-style-guide] for commit messages:
  - Capitalized, short (50 characters or less) summary in imperative mode
    ("Fix bug", not "Fixed bug")
  - Blank line between summary and body
  - Focus on the "why" — motivation and reasoning — not what changed
  - Minimal formatting or bullet points, plain prose is preferred
  - Full sentences with simple past and present tense
  - Wrap the body at 72 characters
- Write commit messages for a reader that has no prior context and no access to
  the session history.
- Keep commit messages concise. Aim for two or three paragraphs, not more.
- Don't use backticks in commit message titles, but do use them in bodies.
- **Never** write conventional commit messages.
- **Never** add yourself as a co-author.

[git-style-guide]: https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html
[keep-a-changelog]: https://keepachangelog.com/en/1.0.0/
[trycmd]: https://docs.rs/trycmd/latest/trycmd/
