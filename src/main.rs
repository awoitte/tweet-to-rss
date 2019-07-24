extern crate select;
extern crate reqwest;
extern crate clap;

use select::document::Document;
use select::predicate::Class;
use clap::{Arg, App};

use std::io::{Error, ErrorKind};

fn main() -> Result<(),Box<std::error::Error>> {
    let args = App::new("tweet-to-rss")
        .version("0.1.0")
        .author("Alicia Woitte <aliciawoitte@gmail.com>")
        .about("Scrapes a twitter profile page and converts the tweets into an RSS feed output file")
        .arg(Arg::with_name("username")
             .help("the username of the twitter account to convert")
             .required(true)
             .index(1))
        .get_matches();

    let username = args.value_of("username").unwrap();

    let user_url = format!("https://twitter.com/{}", username);
    let mut response = match reqwest::get(&user_url) {
        Ok(response) => response,
        Err(e) => {
                eprintln!("{:?}", e);
                let error_string = format!("unable to connect to twitter url {}", user_url);
                return Err(std::boxed::Box::new(Error::new(ErrorKind::Other, error_string)))
            }
    };

    let body : &str = &response.text()?;
    let document = Document::from(body);

    println!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>");
    println!("<rss version=\"2.0\">");
    println!("<channel>");

    println!("<title>{} tweets</title>", username);
    println!("<link>{}</link>", user_url);
    println!("<description>{}</description>", user_url);

    for node in document.find(Class("tweet")) {
        println!("<item>");
        let timestamp = node.find(Class("tweet-timestamp")).next().unwrap();
        let text = node.find(Class("tweet-text")).next().unwrap();
        let context = node.find(Class("context")).next().unwrap();

        println!("<title>{}</title>", timestamp.attr("href").unwrap());

        println!("<link>{}</link>", timestamp.attr("href").unwrap());
        println!("<description><![CDATA[\n\n{}<br>{}<br>{}\n\n]]></description>", timestamp.text(), context.text().trim(), text.text());
        println!("</item>");
        println!("");
    }

    println!("</channel>");
    println!("</rss>");

    Ok(())
}
