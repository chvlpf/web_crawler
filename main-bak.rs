
use reqwest;
use tokio;
use std::error::Error;
use spider::website::Website;
use url::Url;
use scraper::{Html, Selector, ElementRef};
use std::fmt::Write;
use slug::slugify;
use std::fs;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let domain = "https://www.heygoody.com";
    let robots_url = format!("{}/robots.txt", domain);
    let sitemap_fallback_url = format!("{}/sitemap.xml", domain);

    let mut sitemap_seed: Vec<String> = Vec::new();
    let responses_text: String;

    let response = reqwest::get(&robots_url).await;

    let loaded_from: &str;

    responses_text = match response {
        Ok(resp) if resp.status().is_success() => {
            println!("\n\n");
            println!("---------------------------------------------------------");
            println!("loaded robots.txt from {}", robots_url);
            println!("---------------------------------------------------------\n");
            loaded_from = "robots.txt";
            resp.text().await?
        }
        _ => {
            println!("\n\n");
            println!("---------------------------------------------------------");
            eprintln!("failed load robots.txt, trying to load sitemap.xml...");
            println!("---------------------------------------------------------\n");
            let sitemap_resp = reqwest::get(&sitemap_fallback_url).await;
            match sitemap_resp {
                Ok(resp) if resp.status().is_success() => {
                    println!("---------------------------------------------------------");
                    println!("loaded sitemap.xml from {}", sitemap_fallback_url);
                    println!("---------------------------------------------------------\n");
                    loaded_from = "sitemap.xml";
                    format!("Sitemap: {}", sitemap_fallback_url)
                }
                _ => {
                    println!("---------------------------------------------------------");
                    eprintln!("failed load sitemap.xml, native spider crawl from home page...");
                    println!("---------------------------------------------------------\n");

                    let mut website = Website::new(domain);
                    website.configuration.respect_robots_txt = false;
                    website.configuration.depth = 2;
                    website.configuration.delay = 1;
                    website.crawl().await;
                    println!("found {} internal links:", website.get_links().len());
                    for url in website.get_links() {
                        sitemap_seed.push(url.to_string());
                    }
                    loaded_from = "spider";
                    String::new()
                }
            }
        }
    };

    if loaded_from != "spider" {
        println!("---------------------------------------------------------");
        println!("responses_text={}", responses_text);
        println!("---------------------------------------------------------\n");

        let res_sitemap_urls: Vec<String> = responses_text
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.to_lowercase().starts_with("sitemap:") {
                    Some(line["Sitemap:".len()..].trim().to_string())
                } else {
                    None
                }
            })
            .collect();

        println!("---------------------------------------------------------");
        println!("found {} res_sitemap(s):", res_sitemap_urls.len());
        for url in &res_sitemap_urls {
            sitemap_seed.push(url.to_string());
        }
        println!("---------------------------------------------------------\n");
    }

    println!("---------------------------------------------------------");
    for new_url in &sitemap_seed {
        let domain_host = extract_host(new_url).unwrap_or_default();
        let render_mode = get_render_mode(&domain_host);
        println!("domain_host={}",domain_host);
        println!("render_mode={:?}",render_mode);
        if render_mode == RenderMode::SPA {
            let html = reqwest::get(new_url).await?.text().await?;
            let markdown = html_to_markdown(&html);
            save_markdown_file_from_url(new_url, &markdown)?;
        }
    }
    println!("---------------------------------------------------------\n");
    Ok(())
}

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

pub fn html_to_markdown(html: &str) -> String {
    let document = Html::parse_document(html);
    let mut md = String::new();

    for node in document.root_element().children() {
        if let Some(element) = ElementRef::wrap(node) {
            handle_element(&mut md, element);
        }
    }

    md
}

fn handle_element(md: &mut String, el: ElementRef) {
    let tag = el.value().name();

    match tag {
        "h1" => writeln!(md, "# {}", el.text().collect::<String>().trim()).unwrap(),
        "h2" => writeln!(md, "## {}", el.text().collect::<String>().trim()).unwrap(),
        "p" => writeln!(md, "{}", el.text().collect::<String>().trim()).unwrap(),
        "ul" => {
            for li in el.select(&Selector::parse("li").unwrap()) {
                writeln!(md, "- {}", li.text().collect::<String>().trim()).unwrap();
            }
        }
        "ol" => {
            for (i, li) in el.select(&Selector::parse("li").unwrap()).enumerate() {
                writeln!(md, "{}. {}", i + 1, li.text().collect::<String>().trim()).unwrap();
            }
        }
        "a" => {
            let href = el.value().attr("href").unwrap_or("#");
            let text = el.text().collect::<String>().trim().to_string();
            writeln!(md, "[{}]({})", text, href).unwrap();
        }
        "img" => {
            let src = el.value().attr("src").unwrap_or("");
            let alt = el.value().attr("alt").unwrap_or("");
            writeln!(md, "![{}]({})", alt, src).unwrap();
        }
        "strong" => write!(md, "**{}**", el.text().collect::<String>().trim()).unwrap(),
        "em" => write!(md, "_{}_", el.text().collect::<String>().trim()).unwrap(),
        "blockquote" => {
            for line in el.text().collect::<String>().lines() {
                writeln!(md, "> {}", line.trim()).unwrap();
            }
        }
        "br" => writeln!(md).unwrap(),

        _ => {
            // รีเคอร์ซีฟ traverse
            for child in el.children() {
                if let Some(child_el) = ElementRef::wrap(child) {
                    handle_element(md, child_el);
                }
            }
        }
    }
}


pub fn save_markdown_file_from_url(url: &str, content: &str) -> std::io::Result<()> {
    let slug = url_to_slug(url);
    let folder = "output/";
    let path = format!("{}{}.md", folder, slug);

    fs::create_dir_all(folder)?; // สร้าง output/ ถ้ายังไม่มี
    fs::write(&path, content)?;
    println!("📄 saved to {}", path);
    Ok(())
}

fn url_to_slug(url: &str) -> String {
    let parsed = Url::parse(url).ok();
    if let Some(path) = parsed.map(|u| u.path().trim_matches('/').to_string()) {
        if path.is_empty() {
            "index".to_string()
        } else {
            slugify(path)
        }
    } else {
        "unknown".to_string()
    }
}
