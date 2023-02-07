<img src="https://raw.githubusercontent.com/Lozul/glessite/main/static/images/glessite_logo.svg" alt="Glessite" width=565>

[![Rust](https://github.com/Lozul/glessite/actions/workflows/rust.yml/badge.svg)](https://github.com/Lozul/glessite/actions/workflows/rust.yml)

A simple static site generator based on commits from a git repository.

## How to use

The program expect to be launched within a git repository.
If so, it will walk through the commit history and pick those whose title are
prefixed by `POST: `.
For every "post" found, an entry will be added to the index page, and a simple
HTML page will be generated using the title and the body (if present) of the
commit.

All the generated files will be stored in a `public` folder in the current
working directory.

### CLI

Glessite offers some options:

```
-r, --repository <REPOSITORY>  Repository to use, default to current working dir, must be a valid path to a directory containing a .git
-o, --output-dir <OUTPUT_DIR>  Output directory, default to `public`
-p, --prefix <PREFIX>          Prefix to filter posts from normal commits, detault to `POST: `
-n, --no-prefix                If present, every commit will be used, prefix option is ignored
```

### Workflow example

1. Create a "post" commit

```
# Inside a git repository
git commit --allow-empty -m "POST: Title" -m "Body of the post"
```

2. Generate the site

```
glessite
```

3. View the result by opening the generate web pages in a browser, for example

```
firefox public/index.html
# or
python -m http.server --directory public
```

## How to build and install

1. Install Rust, this documentation assume that you have access to the `cargo`
   command

### With [crates.io](https://crates.io/crates/glessite)

2. Install the crate

```
cargo install glessite
```

### Manually
2. Download the repository and compile:

```
git clone https://github.com/Lozul/glessite
cd glessite
cargo build --release
```

3. Install the crate

```
cargo install --path .
```

### How to uninstall

```
cargo uninstall glessite
```

## License

This projet is under the MIT License.

The logo use the font [Source Code
Pro](https://github.com/adobe-fonts/source-code-pro) released under the
SIL Open Font License 1.1.

## Where does the name come from

Glessite is a type of natural resin, found it on this
[page](https://en.wikipedia.org/wiki/List_of_minerals).
The name started with _G_ like Git and ended with _site_, which fitted with the
project being a site generator.
