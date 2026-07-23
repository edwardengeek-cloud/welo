#![no_std]
use aidoku::{
    error::Result, prelude::*, std::net::HttpMethod, std::net::Request, std::String, std::Vec,
    Chapter, DeepLink, Filter, Manga, MangaPageResult, Page,
};

mod parser;
use parser::BASE_URL;

#[get_manga_list]
fn get_manga_list(_filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
    let url = format!("{}/manga-list.html?sort=last_update&sort_type=DESC&page={}", BASE_URL, page);
    let html = Request::new(url.as_str(), HttpMethod::Get).html()?;
    parser::parse_manga_list(html)
}

#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
    let url = format!("{}{}", BASE_URL, if manga_id.starts_with('/') { manga_id } else { format!("/{}", manga_id) });
    let html = Request::new(url.as_str(), HttpMethod::Get).html()?;
    parser::parse_manga_details(html, manga_id)
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
    let url = format!("{}{}", BASE_URL, if manga_id.starts_with('/') { manga_id } else { format!("/{}", manga_id) });
    let html = Request::new(url.as_str(), HttpMethod::Get).html()?;
    parser::parse_chapter_list(html)
}

#[get_page_list]
fn get_page_list(_manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
    let url = format!("{}{}", BASE_URL, if chapter_id.starts_with('/') { chapter_id } else { format!("/{}", chapter_id) });
    let html = Request::new(url.as_str(), HttpMethod::Get).html()?;
    parser::parse_page_list(html)
}

#[handle_url]
pub fn handle_url(url: String) -> Result<DeepLink> {
    Ok(DeepLink {
        manga: Some(get_manga_details(url.clone())?),
        chapter: None,
    })
}