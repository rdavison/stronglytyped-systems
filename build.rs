use std::collections::HashMap;
use std::fs;
use std::path::Path;

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

#[derive(serde::Deserialize)]
struct Frontmatter {
    title: String,
    date: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    description: String,
    #[serde(default)]
    draft: bool,
}

fn main() {
    println!("cargo:rerun-if-changed=content/posts");

    let posts_dir = Path::new("content/posts");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("posts_data.rs");

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    let mut posts = Vec::new();

    if posts_dir.exists() {
        for entry in fs::read_dir(posts_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.extension().is_some_and(|e| e == "md") {
                let content = fs::read_to_string(&path).unwrap();
                let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
                let parsed = matter.parse(&content);

                let frontmatter: Frontmatter = parsed
                    .data
                    .as_ref()
                    .and_then(|d| {
                        let yaml_str = match d {
                            gray_matter::Pod::Hash(map) => {
                                pod_to_yaml(map)
                            }
                            _ => String::new(),
                        };
                        serde_yaml::from_str(&yaml_str).ok()
                    })
                    .unwrap_or_else(|| {
                        panic!("Failed to parse frontmatter in {:?}", path);
                    });

                if frontmatter.draft {
                    continue;
                }

                let slug = path
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();

                let html = render_markdown(&parsed.content, &ss, theme);

                posts.push((slug, frontmatter, html));
            }
        }
    }

    // Sort by date descending
    posts.sort_by(|a, b| b.1.date.cmp(&a.1.date));

    let mut code = String::new();
    code.push_str("&[\n");

    for (slug, fm, html) in &posts {
        let tags_str = fm
            .tags
            .iter()
            .map(|t| format!("\"{}\"", t.replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join(", ");

        code.push_str(&format!(
            "    Post {{\n        slug: \"{}\",\n        title: \"{}\",\n        date: \"{}\",\n        description: \"{}\",\n        tags: &[{}],\n        html: r##\"{}\"##,\n    }},\n",
            slug.replace('"', "\\\""),
            fm.title.replace('"', "\\\""),
            fm.date.replace('"', "\\\""),
            fm.description.replace('"', "\\\""),
            tags_str,
            html,
        ));
    }

    code.push_str("]\n");

    fs::write(&out_path, code).unwrap();
}

fn pod_to_yaml(map: &HashMap<String, gray_matter::Pod>) -> String {
    let mut yaml = String::new();
    for (key, value) in map {
        match value {
            gray_matter::Pod::String(s) => {
                yaml.push_str(&format!("{}: \"{}\"\n", key, s));
            }
            gray_matter::Pod::Boolean(b) => {
                yaml.push_str(&format!("{}: {}\n", key, b));
            }
            gray_matter::Pod::Integer(i) => {
                yaml.push_str(&format!("{}: {}\n", key, i));
            }
            gray_matter::Pod::Float(f) => {
                yaml.push_str(&format!("{}: {}\n", key, f));
            }
            gray_matter::Pod::Array(arr) => {
                yaml.push_str(&format!("{}:\n", key));
                for item in arr {
                    if let gray_matter::Pod::String(s) = item {
                        yaml.push_str(&format!("  - \"{}\"\n", s));
                    }
                }
            }
            gray_matter::Pod::Hash(h) => {
                yaml.push_str(&format!("{}:\n", key));
                let inner = pod_to_yaml(h);
                for line in inner.lines() {
                    yaml.push_str(&format!("  {}\n", line));
                }
            }
            gray_matter::Pod::Null => {
                yaml.push_str(&format!("{}: null\n", key));
            }
        }
    }
    yaml
}

fn render_markdown(content: &str, ss: &SyntaxSet, theme: &syntect::highlighting::Theme) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(content, options);

    let mut html_output = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_text = String::new();

    let events: Vec<Event> = parser.collect();

    for event in events {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_text.clear();
                code_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                let syntax = if code_lang.is_empty() {
                    ss.find_syntax_plain_text()
                } else {
                    ss.find_syntax_by_token(&code_lang)
                        .unwrap_or_else(|| ss.find_syntax_plain_text())
                };

                match highlighted_html_for_string(&code_text, ss, syntax, theme) {
                    Ok(highlighted) => {
                        html_output.push_str("<div class=\"code-block\">");
                        if !code_lang.is_empty() {
                            html_output.push_str(&format!(
                                "<span class=\"code-block__lang\">{}</span>",
                                code_lang
                            ));
                        }
                        html_output.push_str(&highlighted);
                        html_output.push_str("</div>");
                    }
                    Err(_) => {
                        html_output.push_str("<pre><code>");
                        html_output.push_str(&html_escape(&code_text));
                        html_output.push_str("</code></pre>");
                    }
                }
            }
            Event::Text(text) => {
                if in_code_block {
                    code_text.push_str(&text);
                } else {
                    html_output.push_str(&html_escape(&text));
                }
            }
            Event::Code(text) => {
                html_output.push_str("<code class=\"inline-code\">");
                html_output.push_str(&html_escape(&text));
                html_output.push_str("</code>");
            }
            Event::Start(Tag::Heading { level, .. }) => {
                html_output.push_str(&format!("<h{}>", heading_level_num(level)));
            }
            Event::End(TagEnd::Heading(level)) => {
                html_output.push_str(&format!("</h{}>", heading_level_num(level)));
            }
            Event::Start(Tag::Paragraph) => {
                html_output.push_str("<p>");
            }
            Event::End(TagEnd::Paragraph) => {
                html_output.push_str("</p>");
            }
            Event::Start(Tag::List(None)) => {
                html_output.push_str("<ul>");
            }
            Event::End(TagEnd::List(false)) => {
                html_output.push_str("</ul>");
            }
            Event::Start(Tag::List(Some(start))) => {
                html_output.push_str(&format!("<ol start=\"{}\">", start));
            }
            Event::End(TagEnd::List(true)) => {
                html_output.push_str("</ol>");
            }
            Event::Start(Tag::Item) => {
                html_output.push_str("<li>");
            }
            Event::End(TagEnd::Item) => {
                html_output.push_str("</li>");
            }
            Event::Start(Tag::Strong) => {
                html_output.push_str("<strong>");
            }
            Event::End(TagEnd::Strong) => {
                html_output.push_str("</strong>");
            }
            Event::Start(Tag::Emphasis) => {
                html_output.push_str("<em>");
            }
            Event::End(TagEnd::Emphasis) => {
                html_output.push_str("</em>");
            }
            Event::Start(Tag::Link { dest_url, title, .. }) => {
                html_output.push_str(&format!("<a href=\"{}\"", dest_url));
                if !title.is_empty() {
                    html_output.push_str(&format!(" title=\"{}\"", html_escape(&title)));
                }
                html_output.push('>');
            }
            Event::End(TagEnd::Link) => {
                html_output.push_str("</a>");
            }
            Event::Start(Tag::Image { dest_url, title, .. }) => {
                html_output.push_str(&format!(
                    "<figure><img src=\"{}\" alt=\"{}\" class=\"blog-post__image\" loading=\"lazy\" />",
                    dest_url,
                    html_escape(&title)
                ));
            }
            Event::End(TagEnd::Image) => {
                html_output.push_str("</figure>");
            }
            Event::Start(Tag::BlockQuote(_)) => {
                html_output.push_str("<blockquote>");
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                html_output.push_str("</blockquote>");
            }
            Event::SoftBreak => {
                html_output.push('\n');
            }
            Event::HardBreak => {
                html_output.push_str("<br />");
            }
            Event::Rule => {
                html_output.push_str("<hr />");
            }
            _ => {}
        }
    }

    html_output
}

fn heading_level_num(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
