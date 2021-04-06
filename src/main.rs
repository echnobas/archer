
// Archer fetches the latest news from archlinux.org regarding packages managed by the pacman package manager
// Copyright (C) 2021  echnobas

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![warn(clippy::all, clippy::pedantic)]
use feed_rs::parser;
use regex::Regex;
use colored::Colorize;

#[derive(Debug)]
struct ArchEntry {
    pub title: String,
    pub summary: String,
    published: String
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let data = ureq::get("https://www.archlinux.org/feeds/news/").call()?.into_string()?;
    let re = Regex::new("<.*?>").unwrap();
    let feed = parser::parse(data.as_bytes()).unwrap();
    let entries = {
        let mut tmp = Vec::new();
        for ref entry in feed.entries {
            tmp.push(ArchEntry {
                title: entry.title.as_ref().map_or("unknown".to_owned(), |entry| entry.content.to_owned()),
                summary: entry.summary.as_ref().unwrap().content.to_owned(),
                published: entry.published.map_or("unknown".to_owned(), |time| time.to_string())
            })
        }
        tmp
    };
    for item in entries.iter().take(std::env::args().nth(1).unwrap_or_else(|| "3".to_owned()).parse::<usize>()?) {
        println!("{} @ {}
{}\n\n", item.title.red().underline(), item.published.blue().underline(), re.replace_all(&item.summary, "").green());
    }
    Ok(())
}

