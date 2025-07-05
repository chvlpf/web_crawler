use reqwest::blocking::get;
use slug::slugify;
use spider::website::Website;
use std::fs;
use url::Url;

#[derive(Debug, PartialEq)]
pub enum RenderMode {
    SSR,
    SPA,
}

fn get_render_mode(domain: &str) -> RenderMode {
    match domain {
        "www.heygoody.com" => RenderMode::SPA,
        "www.example.com" => RenderMode::SSR,
        _ => RenderMode::SSR,
    }
}

fn extract_host(url_str: &str) -> Option<String> {
    Url::parse(url_str).ok()?.host_str().map(|s| s.to_string())
}

fn url_to_slug(url: &str) -> String {
    Url::parse(url)
        .ok()
        .map(|u| {
            let path = u.path().trim_matches('/');
            if path.is_empty() {
                "index".to_string()
            } else {
                slugify(path)
            }
        })
        .unwrap_or_else(|| "unknown".to_string())
}

fn save_markdown_file_from_url(url: &str, content: &str) -> std::io::Result<()> {
    let slug = url_to_slug(url);
    let folder = "output/";
    let path = format!("{}{}.md", folder, slug);

    fs::create_dir_all(folder)?;
    fs::write(&path, content)?;
    println!("Saved to {}", path);
    Ok(())
}

fn simple_html_to_markdown(html: &str) -> String {
    let mut md = String::new();
    let mut tag = String::new();
    let mut inside = false;
    let mut buffer = String::new();

    for c in html.chars() {
        match c {
            '<' => {
                inside = true;
                tag.clear();
                if !buffer.trim().is_empty() {
                    md.push_str(&buffer);
                    buffer.clear();
                }
            }
            '>' => {
                inside = false;
                match tag.to_lowercase().as_str() {
                    "h1" => md.push_str("# "),
                    "/h1" => md.push_str("\n\n"),
                    "h2" => md.push_str("## "),
                    "/h2" => md.push_str("\n\n"),
                    "p" => {},
                    "/p" => md.push_str("\n\n"),
                    "br" => md.push('\n'),
                    _ => {}
                }
            }
            _ => {
                if inside {
                    tag.push(c);
                } else {
                    buffer.push(c);
                }
            }
        }
    }

    if !buffer.is_empty() {
        md.push_str(&buffer);
    }

    md
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let domain = "https://www.heygoody.com";

    let robots_txt_url = format!("{}/robots.txt", domain);
    let sitemap_xml_url = format!("{}/sitemap.xml", domain);

    let mut urls: Vec<String> = vec![];

    println!("Trying robots.txt...");
    if let Ok(resp) = get(&robots_txt_url) {
        if resp.status().is_success() {
            let text = resp.text()?;
            for line in text.lines() {
                if line.to_lowercase().starts_with("sitemap:") {
                    let sitemap_url = line["sitemap:".len()..].trim();
                    urls.push(sitemap_url.to_string());
                }
            }
        }
    }

    if urls.is_empty() {
        println!("Trying sitemap.xml...");
        if let Ok(resp) = get(&sitemap_xml_url) {
            if resp.status().is_success() {
                urls.push(sitemap_xml_url.clone());
            }
        }
    }

    if urls.is_empty() {
        println!(" No sitemap found. Using spider native crawling.");
        let mut website = Website::new(domain);
        website.configuration.depth = 2;
        website.configuration.delay = 1;
        website.configuration.respect_robots_txt = false;
        for u in website.get_links() {
            urls.push(u.to_string());
        }
    }

    println!("🔗 Crawling {} URLs", urls.len());

    for url in urls {
        let host = extract_host(&url).unwrap_or_default();
        let mode = get_render_mode(&host);

        if mode == RenderMode::SSR {
            if let Ok(resp) = get(&url) {
                if resp.status().is_success() {
                    let html = resp.text()?;
                    let md = simple_html_to_markdown(&html);
                    save_markdown_file_from_url(&url, &md)?;
                }
            }
        } else {
            println!("Skipped SPA: {}", url);
        }
    }

    println!("Done.");
    Ok(())
}
