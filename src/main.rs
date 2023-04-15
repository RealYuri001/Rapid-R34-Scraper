#![allow(non_snake_case)]

extern crate clap;

use clap::{Arg, App};
mod requests;
mod json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Tags")
        .version("1.0")
        .about("Scrapes explicit images from a list of sites")
        .arg(Arg::with_name("limit")
                .short("l")
                .long("limit")
                .value_name("LIMIT")
                .takes_value(true)
                .required(false)
                .help("The amount of images you want to get a result. Maximum is 1000")
        ).arg(Arg::with_name("tags")
                .short("t")
                .long("tags")
                .value_name("TAGS")
                .takes_value(true)
                .required(false)
                .help("tags you want to scrape (Split apart by a space)") 
    ).get_matches();
    
    let limit = matches.value_of("limit").unwrap_or("1");

    if limit.to_string().parse::<u32>().unwrap() > 1000 {
        panic!("Limit value must be less than 1000 due to the limit from API.")
    }

    let tags = matches.value_of("tags");
    let actual_tags: String;

    match tags.as_deref() {
        Some(tags) => {
            actual_tags = tags.replace(" ", "+");
        }
        None => actual_tags = "".to_string()
    }

    let url = format!("https://api.rule34.xxx/index.php?page=dapi&s=post&q=index&json=1&limit={}&tags={}", limit, actual_tags);

    println!("---------------------------------------");
    println!("| URL: Original Rule 34");
    println!("| Limit: {}", limit);
    println!("| Tags: {}", actual_tags);
    println!("---------------------------------------");

    println!("Started Scraping... Just wait man, don't rush.");

    let res = requests::get(url).await;

    for txt in res.iter() {
        println!("{:#?}", txt);
    }

    Ok(())
}
