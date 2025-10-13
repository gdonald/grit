# Tooling

## Building the Docs

Install mdBook locally:

```bash
cargo install mdbook
```

Serve with live reload while editing:

```bash
mdbook serve book --open
```

The generated HTML lives in `book/book`. The continuous integration workflow rebuilds the book on every push to `main` and deploys it to GitHub Pages automatically.

## Testing Code Examples

The documentation uses Rust fenced code blocks with the `rust` language tag and `#`-prefixed hidden lines. Running:

```bash
mdbook test book
```

will compile those snippets with `rustdoc`, guarding against regressions as the language evolves.
