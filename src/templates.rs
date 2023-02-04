#[macro_export]
macro_rules! HEADER {
    () => {
        r#"<!DOCTYPE html>
<html lang="en">

    <head>
        <meta charset="utf-8">
        <title>{title}</title>
        <link rel="stylesheet" href="{stylesheet}">
    </head>
"#
    };
}

#[macro_export]
macro_rules! BODY {
    () => {
        r#"
    <body>
        {}
    </body>
"#
    };
}

#[macro_export]
macro_rules! H1 {
    () => {
        r#"
        <h1>{}</h1>
"#
    };
}

#[macro_export]
macro_rules! LIST {
    () => {
        r#"
        <ul>
{}        </ul>
"#
    };
}

#[macro_export]
macro_rules! LIST_ITEM {
    () => {
        r#"            <li><a href="posts/{}.html">{}</a></li>
"#
    };
}

#[macro_export]
macro_rules! A {
    () => {
        r#"<a href="{href}">{text}</a>"#
    };
}

#[macro_export]
macro_rules! P {
    () => {
        r#"
        <p>{}</p>
"#
    };
}

pub const FOOTER: &str = r#"
</html>
"#;

pub const STYLE: &str = r#"
:root {
    --bg: #fbf1c7;
    --bg0_h: #f9f5d7;
    --bg0: #fbf1c7;
    --bg1: #f2e5bc;
    --fg: #3c3836;
    --highlight: #cc241d;
    --gray: #928374;
}

html {
    background: var(--bg);
    color: var(--fg);
}

body {
    width: 50%;
    margin: 0 auto;
    padding: 2em;
    background: var(--bg0_h);
    box-shadow: 2px 2px 2px var(--gray);
}

@media screen and (max-width: 600px) {
    body {
        overflow-x: hidden;
        width: 95%;
        padding: 0.5em;
    }
}
"#;

pub fn render_header(title: &str, stylesheet: &str) -> String {
    format!(HEADER!(), title = title, stylesheet = stylesheet)
}

pub fn render_body(body: &str) -> String {
    format!(BODY!(), body)
}

pub fn render_h1(content: &str) -> String {
    format!(H1!(), content)
}

pub fn render_list(list: &Vec<(String, String)>) -> String {
    let mut lis = String::new();
    for (item, href) in list {
        lis.push_str(format!(LIST_ITEM!(), href, item).as_str());
    }

    format!(LIST!(), lis)
}

pub fn render_a(href: &str, text: &str) -> String {
    render_p(format!(A!(), href = href, text = text).as_str())
}

pub fn render_p(content: &str) -> String {
    format!(P!(), content)
}
