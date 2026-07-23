use aidoku::{error::Result, prelude::*, std::String, std::Vec, Chapter, Manga, MangaPageResult, Page};

pub const BASE_URL: &str = "https://weloma.art";

pub fn parse_manga_list(html: Node) -> Result<MangaPageResult> {
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

pub fn parse_manga_details(html: Node, id: String) -> Result<Manga> {
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

pub fn parse_chapter_list(html: Node) -> Result<Vec<Chapter>> {
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

pub fn parse_page_list(html: Node) -> Result<Vec<Page>> {
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
