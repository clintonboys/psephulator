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

    let tables: Vec<_> = document.select(&table_selector).collect();

    let table_names = ["England", "Scotland", "Wales", "Northern Ireland"];
    let mut constituencies = Vec::new();
    let mut overall_result = HashMap::new();
    let special_constituencies: [&str; 2] = ["Birmingham Hall Green and Moseley", "Bradford West"];

    let table_party_mapping: HashMap<&str, Vec<&str>> = [
        ("England", vec!["LAB", "CON", "REF", "LD", "GRN", "OTH", "X"]),
        ("Scotland", vec!["LAB", "SNP", "CON", "LD", "REF", "GRN", "OTH"]),
        ("Wales", vec!["LAB", "CON", "REF", "PC", "LD", "GRN", "OTH"]),
        ("Northern Ireland", vec!["SF", "DUP", "APNI", "UUP", "SDLP", "TUV", "IND", "OTH"]),
    ].iter().cloned().collect();

    let vote_indices_mapping = [[11, 12, 13, 14, 15, 16, 0],
        [9, 10, 11, 12, 13, 14, 15],
        [9, 10, 11, 12, 13, 14, 15],
        [9, 10, 11, 12, 13, 14, 15],
    ];

    for (i, table) in tables.iter().enumerate() {
        let table_name = table_names[i];
        let party_mapping = table_party_mapping.get(table_name).unwrap();
        let vote_indices = vote_indices_mapping[i];

        for row in table.select(&row_selector).skip(1) {
        // Skip the header row
        let cells: Vec<_> = row.select(&cell_selector).collect();
        if cells.len() < 16 {
            // Ensure the row has enough columns
            continue;
        }

        let constituency_name = cells[0]
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();
        if special_constituencies.contains(&constituency_name.as_str()) {
            if constituency_name.clone() == "Birmingham Hall Green and Moseley".to_string() {
                let mut results = HashMap::new();
                results.insert("LAB".clone().to_string(), 12798 as u32);
                results.insert("Independent1".clone().to_string(), 7142 as u32);
                results.insert("Independent2".clone().to_string(), 6159 as u32);
                results.insert("LD".clone().to_string(), 4711 as u32);
                results.insert("GRN".clone().to_string(), 3913 as u32);
                results.insert("CON".clone().to_string(), 3845 as u32);
                results.insert("REF".clone().to_string(), 2305 as u32);
                results.insert("Independent3".clone().to_string(), 733 as u32);
                constituencies.push(ConstituencyResult {
                    constituency: Constituency {
                        name: constituency_name,
                        candidates: [
                            Candidate {
                                name: "Tahir Ali".to_string(),
                                party: Party {
                                    name: "LAB".to_string(),
                                },
                            },
                            Candidate {
                                name: "Shakeel Afsar".to_string(),
                                party: Party {
                                    name: "Independent1".to_string(),
                                },
                            },
                            Candidate {
                                name: "Mohammad Hafeeze".to_string(),
                                party: Party {
                                    name: "Independent2".to_string(),
                                },
                            },
                            Candidate {
                                name: "Izzy Knowles".to_string(),
                                party: Party {
                                    name: "LD".to_string(),
                                },
                            },
                            Candidate {
                                name: "Zain Ahmed".to_string(),
                                party: Party {
                                    name: "GRN".to_string(),
                                },
                            },
                            Candidate {
                                name: "Henry Morris".to_string(),
                                party: Party {
                                    name: "CON".to_string(),
                                },
                            },
                            Candidate {
                                name: "Stephen McBrine".to_string(),
                                party: Party {
                                    name: "REF".to_string(),
                                },
                            },
                            Candidate {
                                name: "Babar Raja".to_string(),
                                party: Party {
                                    name: "Independent3".to_string(),
                                },
                            },
                        ]
                        .to_vec(),
                    },
                    results,
                });
            } else if constituency_name.clone() == "Bradford West".to_string() {
                let mut results = HashMap::new();
                results.insert("LAB".clone().to_string(), 11724 as u32);
                results.insert("Independent1".clone().to_string(), 11017 as u32);
                results.insert("Independent3".clone().to_string(), 3547 as u32);
                results.insert("LD".clone().to_string(), 756 as u32);
                results.insert("GRN".clone().to_string(), 3690 as u32);
                results.insert("CON".clone().to_string(), 3055 as u32);
                results.insert("REF".clone().to_string(), 2958 as u32);
                results.insert("Independent2".clone().to_string(), 334 as u32);
                constituencies.push(ConstituencyResult {
                    constituency: Constituency {
                        name: constituency_name,
                        candidates: [
                            Candidate {
                                name: "Naz Shah".to_string(),
                                party: Party {
                                    name: "LAB".to_string(),
                                },
                            },
                            Candidate {
                                name: "Muhammed Islam".to_string(),
                                party: Party {
                                    name: "Independent1".to_string(),
                                },
                            },
                            Candidate {
                                name: "Uman Ghafoor".to_string(),
                                party: Party {
                                    name: "Independent2".to_string(),
                                },
                            },
                            Candidate {
                                name: "Imad Uddin Ahmed".to_string(),
                                party: Party {
                                    name: "LD".to_string(),
                                },
                            },
                            Candidate {
                                name: "Khalid Mahmood".to_string(),
                                party: Party {
                                    name: "GRN".to_string(),
                                },
                            },
                            Candidate {
                                name: "Nigel Moxon".to_string(),
                                party: Party {
                                    name: "CON".to_string(),
                                },
                            },
                            Candidate {
                                name: "Jamie Hinton-Wardle".to_string(),
                                party: Party {
                                    name: "REF".to_string(),
                                },
                            },
                            Candidate {
                                name: "Akeel Hussain".to_string(),
                                party: Party {
                                    name: "Independent3".to_string(),
                                },
                            },
                        ]
                        .to_vec(),
                    },
                    results,
                });
            }
        } else {
            let mut candidates = Vec::new();
            let mut results = HashMap::new();

            // let parties = vec!["LAB", "CON", "REF", "LD", "GRN", "OTH"];

            for (i, &vote_index) in vote_indices.iter().enumerate() {
                if vote_indices[i] >= cells.len() {
                    continue;
                }

                if let Some(party_cell) = cells.get(vote_index) {
                    let party_name = party_mapping[i];
                    let votes = party_cell
                        .text()
                        .collect::<Vec<_>>()
                        .join("")
                        .trim()
                        .replace(",", "")
                        .parse()
                        .unwrap_or(0);
                    candidates.push(Candidate {
                        name: format!("Candidate{}", i + 1),
                        party: Party {
                            name: party_name.clone().to_string(),
                        },
                    });
                    *overall_result
                        .entry(party_name.clone().to_string())
                        .or_insert(0) += votes;
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
    }
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
