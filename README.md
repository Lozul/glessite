# Glessite

A simple static site generator based on commits from a git repository.

## How to use

> First of all, this is a work in progress, there is still case in the code
> where the program will panic.

The program expect to be launched within a git repository.
If so, it will walk through the commit history and pick those whose title are
prefixed by `POST: `.
For every "post" found, an entry will be added to the index page, and a simple
HTML page will be generated using the title and the body (if present) of the
commit.

All the generated files will be stored in a `public` folder in the current
working directory.

### Workflow example

1. Create a "post" commit

```
# Inside a git repository
git commit --allow-empty -m "POST: Title" -m "Body of the post"
```

2. Generate the site

```
$ glessite
```

3. View the result by opening the generate web pages in a browser, for example

```
firefox public/index.html
# or
python -m http.server --directory public
```

## How to build and install

**NOTE:** As this projet is more a proof of concept than a finished tool, it is
not published on crates.io.
Therefore, the installation process is manual and without guarantee of success.

1. Install Rust, this documentation assume that you have access to the `cargo`
   command

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

## Roadmap

- Less error prone
    - [x] fail gracefully if cwd is not a repository
    - [x] remove any use of panic at all
- Options
    - [x] choose the repository
    - [x] choose the output path
    - [x] choose the prefix
    - [x] disable the use of a prefix, every commit should be used
- Customisation
    - [ ] overloading of theme
    - [ ] overloading of templates
    - [ ] suppport of light markup (unlikely)

## Where does the name come from

Glessite is a type of natural resin, found it on this
[page](https://en.wikipedia.org/wiki/List_of_minerals).
The name started with _G_ like Git and ended with _site_, which fitted with the
project being a site generator.
