use std::fs;

use log::{error, info};
use reqwest::get;
use scraper::{Html, Selector};

use crate::json_to_structs::recent::Recent;
use crate::scrapers::scraper_resources::resources::{fetch_failed, get_local, request_html};

pub async fn html_processor_wt_forums() -> Option<String> {
	let recent = get_local();

	let url = &recent.forums.domain;

	let html;
	if let Some(value) = request_html(&url).await {
		html = value;
	} else {
		return fetch_failed()
	}

	let top_url_selector = Selector::parse("body > main > div > div > div > div:nth-child(2) > div > ol > li:nth-child(2) > div > h4 > div > a").unwrap();

	return if let Some(top_url) = html.select(&top_url_selector).next() {
		let top_url = top_url.value().attr("href").unwrap();
		Some(top_url.to_string())
	} else {
		println!("Fetch failed");
		error!("Fetch failed");
		None
	};
}