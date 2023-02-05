use clap::Parser;
use env_logger::Builder;
use git2::Repository;
use log::{error, info, warn, LevelFilter};
use std::{
    fs::{create_dir_all, write},
    io::Error,
    path::Path,
    process::exit,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

mod templates;

const REPO_DIR: &str = ".";
const PUBLIC_DIR: &str = "public";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Repository to use, default to current working dir,
    /// must be a valid path to a directory containing a .git
    #[arg(short, long)]
    repository: Option<String>,

    /// Output directory, default to `public`
    #[arg(short, long)]
    output_dir: Option<String>,
}

fn main() {
    setup_logger();

    let cli = Cli::parse();

    let repository = cli.repository.unwrap_or_else(|| REPO_DIR.to_string());
    info!("Repository path: {}", repository);

    let output_dir = cli.output_dir.unwrap_or_else(|| PUBLIC_DIR.to_string());
    info!("Output directory: {}", output_dir);

    let (sendr, recvr) = channel::<git2::Oid>();

    if let Err(e) = create_dir_all(Path::new(&output_dir).join("posts")) {
        error!("Failed to create output folder: {}", e);
        exit(1);
    }

    let handler = {
        let repository = repository.clone();

        thread::spawn(move || {
            ensurer(&recvr, &repository, &output_dir);
        })
    };

    browser(&sendr, &repository);
    drop(sendr);

    if handler.join().is_err() {
        error!("Failed to join ensurer thread");
    }
}

fn setup_logger() {
    let mut logger_builder = Builder::from_default_env();

    logger_builder
        .format_timestamp(None)
        .filter(None, LevelFilter::Info)
        .init();
}

fn browser(sender: &Sender<git2::Oid>, input_dir: &str) {
    let repo = match Repository::open(input_dir) {
        Ok(repo) => repo,
        Err(e) => {
            error!("Failed to open repo in browser: {}", e);
            exit(1);
        }
    };

    let mut revwalk = match repo.revwalk() {
        Ok(revwalk) => revwalk,
        Err(e) => {
            error!("Failed to init revwalk: {}", e);
            exit(1);
        }
    };

    if let Err(e) = revwalk.push_head() {
        error!("Failed to push head to revwalk: {}", e);
        exit(1);
    }

    if let Err(e) = revwalk.set_sorting(git2::Sort::TIME) {
        warn!("Failed to set sorting method of revwalk: {}", e);
    };

    for rev in revwalk {
        let oid = match rev {
            Ok(oid) => oid,
            Err(e) => {
                error!("{}", e);
                continue;
            }
        };

        if let Err(e) = sender.send(oid) {
            error!("Failed to send oid to ensurer thread: {}", e);
            continue;
        }
    }
}

fn ensurer(receiver: &Receiver<git2::Oid>, input_dir: &str, output_dir: &str) {
    let repo = match Repository::open(input_dir) {
        Ok(repo) => repo,
        Err(e) => {
            error!("Failed to open repo in ensurer: {}", e);
            exit(1);
        }
    };

    let mut index: Vec<(String, String)> = Vec::new();

    while let Ok(oid) = receiver.recv() {
        let commit = match repo.find_commit(oid) {
            Ok(commit) => commit,
            Err(e) => {
                error!("Failed to find commit: {}", e);
                continue;
            }
        };

        let mut title = match commit.summary() {
            Some(title) => title,
            None => continue,
        };

        if !title.starts_with("POST: ") {
            continue;
        }

        title = match title.strip_prefix("POST: ") {
            Some(title) => title,
            None => continue,
        };

        let body = commit.body().unwrap_or("");

        let short_oid = shorten_oid(&oid);

        if let Err(e) = write_post(&short_oid, title, body, output_dir) {
            error!("Failed to write post ({}): {}", short_oid, e);
            continue;
        }

        info!("Post {} generated", short_oid);

        index.push((title.to_string(), short_oid.clone()));
    }

    if let Err(e) = write_index(&index, output_dir) {
        error!("Failed to write index: {}", e);
    } else {
        info!("Index generated");
    }

    if let Err(e) = write_stylesheet(output_dir) {
        error!("Failed to write style sheet: {}", e);
    } else {
        info!("Style sheet generated");
    }
}

fn shorten_oid(oid: &git2::Oid) -> String {
    let fmt_oid = format!("{}", oid);
    let (short, _) = fmt_oid.split_at(7);

    short.to_owned()
}

fn write_index(posts_list: &Vec<(String, String)>, output_dir: &str) -> Result<(), Error> {
    let mut html = templates::render_header("Posts Index", "style.css");

    let mut body = templates::render_h1("Posts Index");
    body.push_str(templates::render_list(posts_list).as_str());

    html.push_str(templates::render_body(&body).as_str());
    html.push_str(templates::FOOTER);

    let index_path = Path::new(&output_dir).join("index.html");
    write(index_path, html)?;

    Ok(())
}

fn write_post(oid: &str, title: &str, body: &str, output_dir: &str) -> Result<(), Error> {
    let mut html = templates::render_header(title, "../style.css");

    let mut content = templates::render_a("../index.html", "Back");
    content.push_str(templates::render_h1(title).as_str());

    for line in body.split("\n\n") {
        if line.is_empty() {
            continue;
        }

        content.push_str(templates::render_p(line).as_str());
    }

    html.push_str(templates::render_body(&content).as_str());
    html.push_str(templates::FOOTER);

    let filename = oid.to_owned() + ".html";
    let post_path = Path::new(&output_dir).join("posts").join(filename);
    write(post_path, html)?;

    Ok(())
}

fn write_stylesheet(output_dir: &str) -> Result<(), Error> {
    let content = templates::STYLE.to_owned();
    let style_path = Path::new(&output_dir).join("style.css");

    write(style_path, content)
}
