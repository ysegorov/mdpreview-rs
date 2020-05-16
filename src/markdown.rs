use pulldown_cmark::{html, Parser, Options};


pub fn to_html(title: &str, text: &str) -> String {

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(text, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    format!("<html>\n\
        <head>\n\
        <meta charset=\"utf-8\">\n\
        <title>{}</title>\n\
        <link rel=\"stylesheet\" type=\"text/css\" href=\"/github-markdown.css\">\n\
        </head>\n\
        <body>\n\
        <div class=\"markdown-body\">{}</div>\n\
        </body>\n\
        </html>", title, html_output)
}
