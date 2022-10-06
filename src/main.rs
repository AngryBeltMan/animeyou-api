#[macro_use] extern crate rocket;
extern crate reqwest;
use serde::{Serialize};
use scraper::{Html,Selector};
#[derive(Serialize)]
struct Anime {
    title:String,
    id:String
}
#[derive(Serialize)]
struct Episode {
    episode:String,
    id:String
}
#[derive(Serialize)]
struct EpisodePlayer {
    src:String
}
#[get("/<search>")]
async fn search(search:String) -> String {
    let url = format!("https://gogoanime.gold/search.html?keyword={}",search.replace(" ", "+"));
    if let Ok(o) = reqwest::get(url.clone()).await {
        if let Ok(t) = o.text().await {
            let html = Html::parse_document(&t);
            let videos = Selector::parse("#content div div strong div article div a");
            let h = &html;
            let videos:Vec<Anime> = h.select(&videos.unwrap()).into_iter().map(|s| {
                let t = s.value().attr("href").unwrap().to_string();
                let a = Anime { title: t.replace("-", " ").replace("/anime/", ""), id: t.replace("/anime/", "") };
                a
            }).collect();
            let json = match serde_json::to_string_pretty(&videos) {
                Ok(o) => o,
                Err(_) => {
                    serde_json::to_string("Error parsing json value").unwrap()
                }
            };
            json
        }else {
            return "failure".to_string();
        }
    }else {
        "failure".to_string()
    }
}
#[get("/episodes/<url>")]
async fn get_episodes(url:String) -> String {
    let url = format!("https://gogoanime.gold/anime/{}",url);
    if let Ok(o) = reqwest::get(url.clone()).await {
        if let Ok(t) = o.text().await {
            let html = Html::parse_document(&t);
            let episodes_selector = Selector::parse("div.col-6 p");
            let h = &html;
            let videos:Vec<String> = h.select(&episodes_selector.unwrap()).map(|s| {
                s.text().collect::<String>()
            }).collect();
            let mut episodes = vec![];
            for ep in 1..=videos[3].parse::<usize>().unwrap_or_else(|_| return 1) {
                let e = Episode {
                    episode:ep.to_string(),
                    id:format!("{}-episode-{}",url.replace("https://gogoanime.gold/anime/", ""),ep)
                };
                episodes.push(e);
            }
            let json = match serde_json::to_string_pretty(&episodes) {
                Ok(o) => o,
                Err(_) => {
                    serde_json::to_string("Error parsing json value").unwrap()
                }
            };
            json
        }else {
            return "failure".to_string();
        }
    }else {
        "failure".to_string()
    }
}
#[get("/")]
fn home_page() -> String {
    String::from("hello!") // just tests to see if the deploy works
}
#[get("/player/<link>")]
async fn get_video_src(link:String) -> String {
    let url = format!("https://gogoanime.gold/watch/{}",link);
    if let Ok(o) = reqwest::get(url.clone()).await {
        if let Ok(t) = o.text().await {
            let html = Html::parse_document(&t);
            let player_selector = Selector::parse("div#video iframe");
            let h = &html;
            let player:Vec<EpisodePlayer> = h.select(&player_selector.unwrap()).into_iter().map(|s| {
                let t = s.value().attr("src").unwrap().to_string();
                let a = EpisodePlayer {
                    src:t
                };
                a
            }).collect();
            let json = match serde_json::to_string_pretty(&player) {
                Ok(o) => o,
                Err(_) => {
                    serde_json::to_string("Error parsing json value").unwrap()
                }
            };
            json
        }else {
            return "failure".to_string();
        }
    }else {
        "failure".to_string()
    }
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/a",routes![search,get_episodes,get_video_src,home_page])
}
