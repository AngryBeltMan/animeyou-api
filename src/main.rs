#[macro_use] extern crate rocket; // brings all the rocket macros into scope
use rocket::serde::json::Json; // returns a json value
extern crate reqwest; // makes the request to gogoanime
use serde::{Serialize,Deserialize}; // Required to turn a struct into json

use scraper::{Html,Selector}; // scrapes gogo anime for the data
// holds the title of the anime when the api is used for searching
#[derive(Serialize,Debug,Deserialize)]
struct Anime {
    title:String,
    id:String
}
// gets the episode id for the anime 
#[derive(Serialize,Deserialize,Debug)]
struct Episode {
    episode:String,
    id:String
}
// returns the anime player 
#[derive(Serialize,Deserialize,Debug)]
struct EpisodePlayer {
    src:String
}
// returns a vector with all of the animes from a query
#[derive(Deserialize,Serialize,Debug)]
struct AnimeResults {
    status:u16,
    res:Vec<Anime>
}
// returns all of the episodes of a specific anime
#[derive(Deserialize,Serialize,Debug)]
struct EpisodeResults {
    status:u16,
    res:Vec<Episode>
}
// returns a link to watch the anime
// there should currently only return one result
#[derive(Deserialize,Serialize,Debug)]
struct PlayerResults {
    status:u16,
    res:Vec<EpisodePlayer>
}
// enter a search query and it will return the results it got from gogoanime
#[get("/search/<search>")]
async fn search(search:String) -> Json<AnimeResults> {
    let url = format!("https://gogoanime.gold/search.html?keyword={}",search.replace(" ", "+")); // makes the search request
    if let Ok(o) = reqwest::get(url.clone()).await { // makes sure it is sucessful
        if let Ok(t) = o.text().await { // makes sure it can turn the html into a string
            let html = Html::parse_document(&t); // parses the string for further scraping
            let videos = Selector::parse("#content div div strong div article div a"); // CS Selector to grab the results
            let h = &html; // create a pointer to html that will later be consumed
            let videos:Vec<Anime> = h.select(&videos.unwrap())
            .into_iter() // consume the varable h and return an iterator
            .map(|s| {
                let t = s.value().attr("href").unwrap().to_string();
                let a = Anime { title: t.replace("-", " ").replace("/anime/", ""), id: t.replace("/anime/", "") };
                a
            }).collect();
            Json(AnimeResults { status: 200, res: videos }) // if all goes will it will return the json
        }else { // if the formating to a String failed
            return Json(AnimeResults { status: 400, res: vec![Anime {title:String::from("Could not format html into a String!"),id:String::from("failure")}] });
        }
    }else { // could not make the request
        return Json(AnimeResults { status: 400, res: vec![Anime {title:String::from("Could not make the request"),id:String::from("failure")}] });
    }
}
// gets all of the episodes of a certine anime
#[get("/episodes/<url>")]
async fn get_episodes(url:String) -> Json<EpisodeResults> {
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
            return Json(EpisodeResults {status:200,res:episodes});
        }else {
            return Json(EpisodeResults {status:200,res:vec![Episode {episode:String::from("failure"),id:String::from("failure")}]});
        }
    }else {
        return Json(EpisodeResults {status:200,res:vec![Episode {episode:String::from("failure"),id:String::from("failure")}]});
    }
}
#[get("/player/<link>")]
async fn get_video_src(link:String) -> Json<PlayerResults> {
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
            Json(PlayerResults { status: 200, res: player })
        }else {
            return Json(PlayerResults { status: 400, res: vec![EpisodePlayer {src:String::from("failure")}] });
        }
    }else {
        return Json(PlayerResults { status: 400, res: vec![EpisodePlayer {src:String::from("failure")}] });
    }
}
#[get("/")]
fn main_page() -> Json<String> {
    Json(String::from("Welcome!\n\n\tThis is an api that was made with rust🦀\n\tUse /help on how to use this api"))
}
#[get("/help")]
fn help() -> Json<String> {
    Json(String::from("use /a/search/YOUR_SEARCH_HERE to search for an anime\n use /a/episodes/YOUR"))
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/a",routes![search,get_episodes,get_video_src]).mount("/", routes![main_page,help])
}
