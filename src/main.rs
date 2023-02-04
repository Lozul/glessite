use git2::Repository;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{
    fs::{create_dir_all, write},
    io::Error,
    path::Path,
    thread,
};

mod templates;

const PUBLIC_DIR: &str = "public";

fn main() {
    let (sendr, recvr) = channel::<git2::Oid>();

    if create_dir_all(Path::new(PUBLIC_DIR).join("posts")).is_err() {
        panic!("failed to create output folder");
    }

    let handler = thread::spawn(move || {
        ensurer(&recvr, PUBLIC_DIR);
    });

    browser(&sendr);
    drop(sendr);

    if handler.join().is_err() {
        panic!("cpt");
    }
}

fn browser(sender: &Sender<git2::Oid>) {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

    let mut revwalk = match repo.revwalk() {
        Ok(revwalk) => revwalk,
        Err(e) => panic!("failed to init revwalk: {}", e),
    };

    if revwalk.push_head().is_err() {
        panic!("failed to push head to revwalk");
    }

    if revwalk.set_sorting(git2::Sort::TIME).is_err() {
        panic!("failed to set sorting method of revwalk");
    };

    for rev in revwalk {
        let oid = match rev {
            Ok(oid) => oid,
            Err(e) => panic!("{}", e),
        };

        if sender.send(oid).is_err() {
            panic!("failed to send oid");
        }
    }
}

fn ensurer(receiver: &Receiver<git2::Oid>, output_dir: &str) {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

    let mut index: Vec<(String, String)> = Vec::new();

    while let Ok(oid) = receiver.recv() {
        let commit = match repo.find_commit(oid) {
            Ok(commit) => commit,
            Err(e) => panic!("failed to find commit: {}", e),
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
            eprintln!("write post failed: {}", e);
        }

        index.push((title.to_string(), short_oid.clone()));
    }

    if let Err(e) = write_index(&index, output_dir) {
        eprintln!("failed to write index: {}", e);
    }

    if let Err(e) = write_stylesheet(output_dir) {
        eprintln!("failed to write stylesheet: {}", e);
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
