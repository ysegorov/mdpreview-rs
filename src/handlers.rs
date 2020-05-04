use std::{fs, io};
use std::path::Path;
use std::ffi::OsStr;

use hyper::header;
use hyper::{Body, Request, Response, StatusCode, Method};

use crate::markdown;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

static NOT_FOUND: &[u8] = b"Not found";

const GITHUB_MARKDOWN_CSS: &str = include_str!("github-markdown.css");


async fn method_not_allowed() -> Result<Response<Body>> {
    let res = Response::builder()
        .header(header::CONTENT_TYPE, "text/plain")
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::empty())
        .expect("Could not prepare response");
    Ok(res)
}

async fn not_found() -> Result<Response<Body>> {
    let res = Response::builder()
        .header(header::CONTENT_TYPE, "text/plain")
        .status(StatusCode::NOT_FOUND)
        .body(NOT_FOUND.into())
        .expect("Could not prepare response");
    Ok(res)
}

async fn github_markdown_css(method: &Method) -> Result<Response<Body>> {
    let len = GITHUB_MARKDOWN_CSS.len();
    let body = if method == &Method::GET { GITHUB_MARKDOWN_CSS.into() } else { Body::empty() };
    let res = Response::builder()
        .header(header::CONTENT_TYPE, "text/css")
        .header(header::CONTENT_LENGTH, len)
        .body(body)
        .expect("Could not prepare github-markdown.css response");
    Ok(res)
}

async fn handle_content(method: &Method, path: &Path, rootdir: &Path) -> Result<Response<Body>> {
    let filename = path.file_name().unwrap();
    let ext = path.extension().unwrap();

    if !rootdir.join(filename).is_file() || ext != "md" {
        return not_found().await;
    }
    let content = load_file(rootdir.join(filename).to_str().unwrap());
    let title = filename.to_str().unwrap();

    render(method, title, &content)
}

async fn handle_index(method: &Method, rootdir: &Path) -> Result<Response<Body>> {
    let title = "Markdown files";
    let mut content = String::new();

    content.push_str(&format!("# Markdown files in {}\n\n", rootdir.to_str().unwrap()));

    let mut entries = fs::read_dir(rootdir)?
        .map(|res| res.map(|e| e.path()))
        .filter(|e| e.as_ref().unwrap().extension() == Some(OsStr::new("md")))
        .collect::<std::result::Result<Vec<_>, io::Error>>()?;
    entries.sort();

    for entry in entries {
        let filename = entry.file_name().unwrap().to_str().unwrap();
        let line = format!("- [{}](/{})\n", filename, filename);
        content.push_str(&line);
    }

    render(method, title, &content)
}

fn render(method: &Method, title: &str, content: &str) -> Result<Response<Body>> {
    let content = markdown::to_html(title, content);
    let len = content.len();
    let body = match method {
        &Method::GET => content.into(),
        _ => Body::empty()
    };
    let res = Response::builder()
        .header(header::CONTENT_TYPE, "text/html")
        .header(header::CONTENT_LENGTH, len)
        .body(body)
        .expect("Could not prepare markdown content");
    Ok(res)
}

pub async fn handle_request(req: Request<Body>, rootdir: String) -> Result<Response<Body>> {
    let path = Path::new(req.uri().path());
    let rootdir = Path::new(&rootdir);
    match req.method() {
        &Method::GET | &Method::HEAD => {
            match req.uri().path() {
                "/" => handle_index(req.method(), &rootdir).await,
                "/github-markdown.css" => github_markdown_css(req.method()).await,
                _ => handle_content(req.method(), path, &rootdir).await
            }
        },
        _ => method_not_allowed().await
    }
}

fn load_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(x) => x,
        Err(_) => String::from("unable to load file")
    }
}
