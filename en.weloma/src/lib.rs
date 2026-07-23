#![no_std]
use aidoku::{
    error::Result, prelude::*, std::net::HttpMethod, std::net::Request, std::String, std::Vec,
    Chapter, DeepLink, Filter, Manga, MangaPageResult, Page,
};

const BASE_URL: &str = "https://weloma.art";

#[get_manga_list]
fn get_manga_list(_filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
    let url = format!("{}/manga-list.html?sort=last_update&sort_type=DESC&page={}", BASE_URL, page);
    let html = Request::new(url.as_str(), HttpMethod::Get).html()?;
    parse_manga_list(html)
}

#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
    let url = format!("{}{}", BASE_URL, if manga_id.starts_with('/') { &manga_id } else { &format!("/{}", manga_id) });
    let html = Request::new(url.as_str(), HttpMethod::Get).html()?;
    parse_manga_details(html, manga_id)
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
    let url = format!("{}{}", BASE_URL, if manga_id.starts_with('/') { &manga_id } else { &format!("/{}", manga_id) });
    let html = Request::new(url.as_str(), HttpMethod::Get).html()?;
    parse_chapter_list(html)
}

#[get_page_list]
fn get_page_list(_manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
    let url = format!("{}{}", BASE_URL, if chapter_id.starts_with('/') { &chapter_id } else { &format!("/{}", chapter_id) });
    let html = Request::new(url.as_str(), HttpMethod::Get).html()?;
    parse_page_list(html)
}

#[handle_url]
pub fn handle_url(url: String) -> Result<DeepLink> {
    Ok(DeepLink {
        manga: Some(get_manga_details(url.clone())?),
        chapter: None,
    })
}

// Parser functions
fn parse_manga_list(html: Node) -> Result<MangaPageResult> {
    let mut manga: Vec<Manga> = Vec::new();
    for node in html.select("a[href*='/m/']").array() {
        let node = node.as_node();
        let href = node.attr("href").read();
        let title = node.select("h3, h2, .title").text().read().trim().to_string();
        let cover = node.select("img").attr("src").read();

        if !title.is_empty() {
            manga.push(Manga {
                id: href.clone(),
                title,
                cover_url: if cover.starts_with("http") { cover } else { format!("{}{}", BASE_URL, cover) },
                url: format!("{}{}", BASE_URL, href),
                ..Default::default()
            });
        }
    }
    Ok(MangaPageResult { manga, has_more: true })
}

fn parse_manga_details(html: Node, id: String) -> Result<Manga> {
    let title = html.select("h1, .series-name, .title").text().read().trim().to_string();
    let description = html.select(".description, .summary").text().read().trim().to_string();
    let cover = html.select("img").attr("src").read();

    Ok(Manga {
        id,
        title: if title.is_empty() { "Unknown".to_string() } else { title },
        description: if description.is_empty() { None } else { Some(description) },
        cover_url: if cover.starts_with("http") { cover } else { format!("{}{}", BASE_URL, cover) },
        url: format!("{}{}", BASE_URL, id),
        ..Default::default()
    })
}

fn parse_chapter_list(html: Node) -> Result<Vec<Chapter>> {
    let mut chapters: Vec<Chapter> = Vec::new();
    for node in html.select("a[href*='/c/']").array() {
        let node = node.as_node();
        let href = node.attr("href").read();
        let title = node.select(".chapter-name, .title").text().read().trim().to_string();

        if !href.is_empty() {
            chapters.push(Chapter {
                id: href.clone(),
                title: if title.is_empty() { "Chapter".to_string() } else { title },
                url: format!("{}{}", BASE_URL, href),
                ..Default::default()
            });
        }
    }
    Ok(chapters)
}

fn parse_page_list(html: Node) -> Result<Vec<Page>> {
    let mut pages: Vec<Page> = Vec::new();
    for (i, node) in html.select("img").array().enumerate() {
        let url = node.as_node().attr("src").read().trim().to_string();
        if url.is_empty() || url.contains("data:") { continue; }

        let image_url = if url.starts_with("http") { url } else { format!("{}{}", BASE_URL, url) };

        pages.push(Page {
            index: i as i32,
            url: image_url,
            ..Default::default()
        });
    }
    Ok(pages)
}
