#![no_std]
use aidoku::{
    error::Result, prelude::*, std::net::HttpMethod, std::net::Request, std::String, std::Vec,
    Chapter, DeepLink, Filter, Manga, MangaPageResult, Page,
};

const BASE_URL: &str = "https://weloma.art";

#[get_manga_list]
fn get_manga_list(_filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
    let url = format!("{}/manga-list.html?sort=last_update&sort_type=DESC&page={}", BASE_URL, page);
    let _html = Request::new(url.as_str(), HttpMethod::Get).html()?;
    Ok(MangaPageResult {
        entries: Vec::new(),
        has_next_page: true,
    })
}

#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
    Ok(Manga {
        key: manga_id,
        title: "Test Manga".to_string(),
        ..Default::default()
    })
}

#[get_chapter_list]
fn get_chapter_list(_manga_id: String) -> Result<Vec<Chapter>> {
    Ok(Vec::new())
}

#[get_page_list]
fn get_page_list(_manga_id: String, _chapter_id: String) -> Result<Vec<Page>> {
    Ok(Vec::new())
}

#[handle_url]
pub fn handle_url(url: String) -> Result<DeepLink> {
    Ok(DeepLink {
        manga: None,
        chapter: None,
    })
}
