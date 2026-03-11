# Contributing

All contributions are welcome!

## What you can do

There are many ways to contribute to httpress:

- reporting bugs
- suggesting new features
- writing documentation
- submitting code changes

## Creating issues

- before creating a new issue make sure one doesn't allready exist
- use one of the provided templates depending on the issue type
  (bug report/feature request)

## Making a patch

1. Fork the repository
2. Create a new branch [e.g `git checkout -b feat/your-feature-name`]
3. Write code
4. Use `cargo fmt --all` to format the code
5. Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any warnings
6. Run `cargo test --all-features` and make sure all tests pass
7. Commit your chages (the commit messages should follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)),
   also if your commit targets a specific issue you should reference that in the
   description
8. Push to your fork
9. Create a pull request
