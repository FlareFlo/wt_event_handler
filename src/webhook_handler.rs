use std::fs;

use log::{error, warn};
use serenity::http::Http;

use crate::json_to_structs::recent::Value;
use crate::json_to_structs::webhooks::{FilterType, Hooks, WebhookAuth};

impl Value {
	//Receives latest content and index in recent array (for WT news)
	pub async fn handle_wt_news_webhook(&self, content: &str) {
		let token_raw = fs::read_to_string("assets/discord_token.json").expect("Cannot read file");
		let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

		for (i, hook) in webhook_auth.hooks.iter().enumerate() {
			if let Some(result) = match_filter(content, hook) {
				deliver_webhooks(result, i).await;
			}
		}
	}

	//Receives latest content and index in recent array
	pub async fn handle_simple_webhook(&self, content: &str) {
		let token_raw = fs::read_to_string("assets/discord_token.json").expect("Cannot read file");
		let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

		for i in 0..webhook_auth.hooks.len() {
			deliver_webhooks(content, i).await;
		}
	}
}

fn match_filter<'a>(content: &'a str, hook: &'a Hooks) -> Option<&'a str> {
	let default_keywords = vec![
		"devblog", "event", "maintenance", "major", "trailer", "teaser", "developers",
		"fix", "vehicles", "economy", "changes", "sale", "twitch", "bundles", "development",
		"shop", "pass", "season", "operation", "pass", "summer", "2021", "planned", "bonds", "issues", "technical", "servers",
	];

	let filter = &hook.filter;

	match filter {
		FilterType::Default => {
			for keyword in default_keywords {
				if content.contains(keyword) {
					println!("URL {} matched with default keyword {}", content, keyword);
					warn!("URL {} matched with default keyword {}", content, keyword);
					return Some(content);
				}
			}
			None
		}
		FilterType::Blacklist => {
			let blacklist = &hook.keywords;
			if blacklist.is_empty() {
				return Some(content);
			}
			for keyword in blacklist {
				if content.contains(keyword) {
					return None;
				}
			}
			println!("No blacklisted items found in {}", content);
			warn!("No blacklisted items found in {}", content);
			Some(content)
		}
		FilterType::Whitelist => {
			let whitelist = &hook.keywords;
			for keyword in whitelist {
				if content.contains(keyword) {
					println!("URL {} matched with whitelisted keyword {}", content, keyword);
					warn!("URL {} matched with whitelisted keyword {}", content, keyword);
					return Some(content);
				}
			}
			None
		}
	}
}

//Finally sends the webhook to the servers
async fn deliver_webhooks(content: &str, pos: usize) {
	let token_raw = fs::read_to_string("assets/discord_token.json").expect("Cannot read file");
	let webhook_auth: WebhookAuth = serde_json::from_str(&token_raw).expect("Json cannot be read");

	let uid = webhook_auth.hooks[pos].uid;
	let token = &webhook_auth.hooks[pos].token;

	let my_http_client = Http::new_with_token(token);

	let webhook = match my_http_client.get_webhook_with_token(uid, token).await {
		Err(why) => {
			println!("{}", why);
			error!("{}", why);
			panic!("")
		}
		Ok(hook) => hook,
	};

	webhook.execute(my_http_client, false, |w| {
		w.content(&format!("[{a}]()", a = content));
		w.username("The WT news bot");
		w.avatar_url("https://cdn.discordapp.com/attachments/866634236232597534/868623209631744000/the_news_broke.png");
		w
	}).await.unwrap();
}