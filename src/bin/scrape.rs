use chrono::{DateTime, Utc};
use reqwest::blocking::get;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectionResult {
    pub datetime: DateTime<Utc>,
    pub constituencies: Vec<ConstituencyResult>,
    pub overall_result: HashMap<String, u32>, // Overall result by party
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstituencyResult {
    pub constituency: Constituency,
    pub results: HashMap<String, u32>, // Results by party
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constituency {
    pub name: String,
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Party {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub name: String,
    pub party: Party,
}

#[derive(Debug, Clone)]
pub enum ElectoralSystem {
    FirstPastThePost,
    ProportionalRepresentation,
    // Add more systems here
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://en.wikipedia.org/wiki/Results_of_the_2024_United_Kingdom_general_election";
    let res = get(url)?.text()?;

    let document = Html::parse_document(&res);
    let table_selector = Selector::parse("table.wikitable").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let table = document.select(&table_selector).next().expect("No table found");

    let mut constituencies = Vec::new();
    let mut overall_result = HashMap::new();

    for row in table.select(&row_selector).skip(1) {  // Skip the header row
        let cells: Vec<_> = row.select(&cell_selector).collect();
        if cells.len() < 18 {  // Ensure the row has enough columns
            continue;
        }

        let constituency_name = cells[0].text().collect::<Vec<_>>().join("").trim().to_string();
	println!("{}", constituency_name);
        let mut candidates = Vec::new();
        let mut results = HashMap::new();

        let vote_indices = vec![11, 12, 13, 14, 15, 16];
	let parties = vec!["LAB", "CON", "REF", "LD", "GRN", "OTH"];

        for (i, &vote_index) in vote_indices.iter().enumerate() {
	    println!("{}: {}", i, vote_index);
            if vote_indices[i] >= cells.len() {
                continue;
            }

            if let Some(party_cell) = cells.get(vote_index) {
                let party_name = parties[i];
		println!("{}", party_name);
                    let votes = party_cell.text().collect::<Vec<_>>().join("").trim().replace(",", "").parse().unwrap_or(0);
		    println!("{}", votes);
                    candidates.push(Candidate {
                        name: format!("Candidate{}", i + 1),
                        party: Party { name: party_name.clone().to_string() },
                    });

                    *overall_result.entry(party_name.clone().to_string()).or_insert(0) += votes;
                    results.insert(party_name.clone().to_string(), votes as u32);
            }
        }

        constituencies.push(ConstituencyResult {
            constituency: Constituency {
                name: constituency_name,
                candidates,
            },
            results,
        });
    }

    let election_result = ElectionResult {
        datetime: Utc::now(),
        constituencies,
        overall_result,
    };

    let json_data = to_string_pretty(&election_result)?;
    let mut file = File::create("/Users/clinton/dev/elections/psephulator/data/uk_2024.json")?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}
