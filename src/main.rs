use clap::Parser;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliParameters {
    #[clap(short, long, value_parser)]
    search: String, //to search
}

#[derive(Serialize, Deserialize, Debug)]
struct Phonetics {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Definitions {
    definition: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Meanings {
    partOfSpeech: String,
    definitions: Vec<Definitions>,
    // synonyms: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<Y> {
    word: String,
    meanings: Vec<Y>,
    sourceUrls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    definition: Items<Meanings>,
}

fn print_meaning(meanings: Vec<&Meanings>) {
    for meaning in meanings {
        println!("🔥 {}", meaning.partOfSpeech);
        for definition in &meaning.definitions {
            println!("💿 {}", definition.definition);
        }
        // println!(
        //     "🕺 {}",
        //     track
        //         .album
        //         .artists
        //         .iter()
        //         .map(|artist| artist.name.to_string())
        //         .collect::<String>()
        // );
        // println!("🌎 {}", track.external_urls.spotify);
        println!("---------")
    }
}

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    let args = CliParameters::parse();

    let url = format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{query}",
        // search string
        query = args.search
    );
    let client = reqwest::Client::new();

    // chaining .await will yield our query result
    let response = client
        .get(url)
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                Ok(parsed) => print_meaning(parsed.definition.meanings.iter().collect()),
                //  Ok(parsed) => println!("Success! {:?}", parsed),
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            }; /*   */
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }; /**/
    // println!("{:?}", result);
}
