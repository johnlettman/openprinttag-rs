# Contributing to openprinttag-rs
Thank you for your interest in contributing to **openprinttag-rs**! Contributions are welcome from everyone and anyone. 
By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## Table of Contents
- [How Can I Contribute?](#how-can-i-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Pull Requests](#pull-requests)
- [Project Structure](#project-structure)
- [Development Setup](#development-setup)
- [Programming Standards](#programming-standards)
  - [Per Crate](#per-crate)
- [Commit Message Guidelines](#commit-message-guidelines)
  - [Commit Message Elements](#commit-message-elements)
  - [Commit Message Examples](#commit-message-examples)
- [License](#license)

## How Can I Contribute?
### Reporting Bugs
- Search existing [Issues](https://github.com/johnlettman/openprinttag-rs/issues) to avoid duplicates.
- If not found, [open a new issue](https://github.com/johnlettman/openprinttag-rs/issues/new).
- Use the **Bug Report** template and include:
  - Steps to reproduce
  - Expected vs. actual behavior
  - Rust version (`rustc --version`)
  - Target platform (operating system, architecture, etc.)
  - Relevant logs or error output
  - Crate(s) affected (`openprinttag-core`, `openprinttag-codegen`, etc.)

### Suggesting Enhancements
- Open a [new issue](https://github.com/johnlettman/openprinttag-rs/issues/new) using the **Feature Request** template.
- Clearly describe the use case and benefit.
- If applicable, reference the OpenPrintTag schema.

### Pull Requests
1. Fork and clone the repository.
2. Create a branch:
    ```bash
    git checkout -b feat/your-feature-name
    # or
    git checkout -b fix/issue-number
    ```
3. Make your changes in the appropriate crate(s).
4. Run formatting, linting, and tests:
    ```bash
    just fmt      # runs rustfmt
    just lint     # runs clippy
    just test     # runs cargo test --all
    ```
5. Ensure pre-commit hooks pass (installed via `pre-commit install`).
6. Update documentation (README, crate docs, schema examples).
7. Commit using [Conventional Commits](#commit-message-guidelines).
8. Push and open a PR against `main`.
9. Fill out the PR template.

#### PR Requirements
- All tests pass (`just test`).
- Clippy has no warnings (`just lint`).
- Code is formatted (`just fmt`).
- Documentation is updated.
- Changes respect crate boundaries.

## Project Structure
```
openprinttag-rs/
├── openprinttag-core/     # Core schema types and runtime
├── openprinttag-codegen/  # Parses OpenPrintTag schema → tokenized AST → Rust codegen
├── openprinttag-ffi/      # C FFI bindings using codegen output
└── openprinttag-cli/      # CLI for schema introspection & NFC data tools
```

## Development Setup
```bash
# Clone the repo
git clone https://github.com/johnlettman/openprinttag-rs.git
cd openprinttag-rs

# Install pre-commit (for git hooks)
pre-commit install

# Verify setup
just --list
just test
```

See [`justfile`](justfile) for all available commands:
```bash
just fmt    # Format all code with rustfmt
just lint   # Run clippy across all crates
just test   # Run tests in all crates
just doc    # Build documentation
just build  # Build all crates
```

## Programming Standards
- Follow ([Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)).
- Use `rustfmt` (`just fmt`)
- Run `clippy` (`just lint`)
- Write expressive, documented public APIs
- Include tests for new functionality
- Update `openprinttag-codegen` schema parser if schema syntax changes

### Per crate
- `openprinttag-codegen`: Keep tokenization and codegen phases separate.
- `openprinttag-ffi`: Ensure C safety (`#[no_mangle]`, `repr(C)`). Test with C code.
- `openprinttag-cli`: Use `clap` for arguments. Support `--help` and schema validation.

## Commit Message Guidelines
Use [**Conventional Commits**](https://www.conventionalcommits.org/en/v1.0.0/):

```
<type>(<scope>): <short summary>

[optional body]

[optional footer]
```

### Commit Message Elements
#### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `refactor`: Code refactoring
- `test`: Adding/fixing tests
- `chore`: Tooling, CI, dependencies
- `perf`: Performance improvements

#### Scopes (_optional but recommended_)
- `core`
- `codegen`
- `ffi`
- `cli`
- `schema`
- `docs`
- `ci`

### Commit Message Examples
```
feat(codegen): support nested schema groups
```
```
fix(ffi): prevent double-free in tag reader
```
```
docs(cli): add examples for nfc-dump command
```

## License
By contributing, you agree that your contributions will be licensed under the **MIT License**. 
See [LICENSE](LICENSE) for details.
