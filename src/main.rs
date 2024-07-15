use std::collections::HashMap;
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};

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

pub fn simulate_election(election_result: &ElectionResult, electoral_system: &ElectoralSystem) -> HashMap<String, u32> {
    match electoral_system {
        ElectoralSystem::FirstPastThePost => simulate_first_past_the_post(election_result),
        ElectoralSystem::ProportionalRepresentation => simulate_proportional_representation(election_result),
        // Add more systems here
    }
}

fn simulate_first_past_the_post(election_result: &ElectionResult) -> HashMap<String, u32> {
    let mut seat_wins: HashMap<String, u32> = HashMap::new();
    
    for constituency_result in &election_result.constituencies {
        if let Some((winning_party, _)) = constituency_result.results.iter().max_by_key(|&(_, votes)| votes) {
            *seat_wins.entry(winning_party.clone()).or_insert(0) += 1;
        }
    }
    
    seat_wins
}

fn simulate_proportional_representation(_election_result: &ElectionResult) -> HashMap<String, u32> {
    // Implement Proportional Representation logic here
    HashMap::new()
}

fn main() {
    println!("Welcome to Psephulator");
    println!("----- v 0.1.0 --------");    

    let options = &["Load Election Results", "Simulate an Election"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    match selection {
        0 => {
            load_election_results();
        },
        1 => {
            simulate_an_election();
        },
        _ => unreachable!(),
    }
}

fn load_election_results() {
    let options = &["2024 UK Election"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an election to load")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    let file_path = match selection {
        0 => "data/uk_2024.json",
        _ => unreachable!(),
    };

    let election_result = load_election_data(file_path);
    println!("Loaded Election Result");

    // Now the user can simulate results in a different electoral system
    let electoral_systems = &["First Past The Post", "Proportional Representation"];
    let system_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an electoral system to simulate results")
        .default(0)
        .items(&electoral_systems[..])
        .interact()
        .unwrap();

    let electoral_system = match system_selection {
        0 => ElectoralSystem::FirstPastThePost,
        1 => ElectoralSystem::ProportionalRepresentation,
        _ => unreachable!(),
    };

    let simulated_result = simulate_election(&election_result, &electoral_system);
    println!("Simulated Result: {:?}", simulated_result);
}

fn load_election_data<P: AsRef<Path>>(path: P) -> ElectionResult {
    let file_content = fs::read_to_string(path).expect("Unable to read file");
    serde_json::from_str(&file_content).expect("JSON was not well-formatted")
}

fn simulate_an_election() {
    let options = &["Simulate a two-party FPTP election"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    match selection {
        0 => {
            let election_result = setup_two_party_fptp_election();
            let electoral_systems = &["First Past The Post", "Proportional Representation"];
            let system_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose an electoral system to simulate results")
                .default(0)
                .items(&electoral_systems[..])
                .interact()
                .unwrap();

            let electoral_system = match system_selection {
                0 => ElectoralSystem::FirstPastThePost,
                1 => ElectoralSystem::ProportionalRepresentation,
                _ => unreachable!(),
            };

            let simulated_result = simulate_election(&election_result, &electoral_system);
            println!("Simulated Result: {:?}", simulated_result);
        },
        _ => unreachable!(),
    }
}

fn setup_two_party_fptp_election() -> ElectionResult {
    let party1 = Party { name: String::from("Party A") };
    let party2 = Party { name: String::from("Party B") };

    let candidate1 = Candidate { name: String::from("Alice"), party: party1.clone() };
    let candidate2 = Candidate { name: String::from("Bob"), party: party2.clone() };

    let constituency = Constituency {
        name: String::from("Constituency 1"),
        candidates: vec![candidate1, candidate2],
    };

    let constituency_result = ConstituencyResult {
        constituency: constituency.clone(),
        results: [(party1.name.clone(), 2), (party2.name.clone(), 1)].iter().cloned().collect(),
    };

    ElectionResult {
        datetime: Utc::now(),
        constituencies: vec![constituency_result],
        overall_result: [(party1.name.clone(), 2), (party2.name.clone(), 1)].iter().cloned().collect(),
    }
}
