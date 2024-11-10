use chrono::{DateTime, Utc};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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
    AlternativeVote, // Added for AV system
}

pub fn simulate_election(
    election_result: &ElectionResult,
    electoral_system: &ElectoralSystem,
    preference_flows: Option<HashMap<String, HashMap<String, f32>>>,
) -> HashMap<String, u32> {
    match electoral_system {
        ElectoralSystem::FirstPastThePost => simulate_first_past_the_post(election_result),
        ElectoralSystem::ProportionalRepresentation => {
            simulate_proportional_representation(election_result)
        }
        ElectoralSystem::AlternativeVote => {
            simulate_alternative_vote(election_result, preference_flows.unwrap())
        }
    }
}

fn simulate_first_past_the_post(election_result: &ElectionResult) -> HashMap<String, u32> {
    let mut seat_wins: HashMap<String, u32> = HashMap::new();

    for constituency_result in &election_result.constituencies {
        if let Some((winning_party, _)) = constituency_result
            .results
            .iter()
            .max_by_key(|&(_, votes)| votes)
        {
            *seat_wins.entry(winning_party.clone()).or_insert(0) += 1;
        }
    }

    seat_wins
}

fn simulate_proportional_representation(_election_result: &ElectionResult) -> HashMap<String, u32> {
    // Implement Proportional Representation logic here
    HashMap::new()
}

fn simulate_alternative_vote(
    election_result: &ElectionResult,
    preference_flows: HashMap<String, HashMap<String, f32>>,
) -> HashMap<String, u32> {
    let mut seat_wins: HashMap<String, u32> = HashMap::new();

    for constituency_result in &election_result.constituencies {
        let mut votes = constituency_result.results.clone();
        let mut eliminated = Vec::new();
        while votes.len() > 2 {
            // Find the party with the minimum votes and remove it
            let (min_party, min_votes) = votes
                .iter()
                .min_by_key(|&(_, &votes)| votes)
                .map(|(party, &votes)| (party.clone(), votes))
                .unwrap();

            votes.remove(&min_party);
            eliminated.push(min_party.clone());

            let remaining_parties: Vec<_> = votes.keys().cloned().collect();
            for (party, &party_votes) in preference_flows.get(&min_party).unwrap_or(&HashMap::new())
            {
                if remaining_parties.contains(party) {
                    let additional_votes = ((min_votes as f32) * party_votes).round() as u32;
                    // println!("Party {} gets {} additional votes", party, additional_votes);
                    *votes.get_mut(party).unwrap() += additional_votes;
                }
            }

            // Redistribute votes for the eliminated party proportionally if it was previously allocated to eliminated parties
            let mut redistributed_votes = 0;
            if !eliminated.is_empty() {
                for (party, &percentage) in
                    preference_flows.get(&min_party).unwrap_or(&HashMap::new())
                {
                    if eliminated.contains(&party) {
                        let redistributed = ((min_votes as f32) * percentage).round() as u32; 
                        // println!("Redistributed {} for {}", redistributed, party);
                        redistributed_votes += redistributed;
                    }
                }

                let total_remaining_percentage: f32 = remaining_parties
                    .iter()
                    .map(|p| {
                        preference_flows
                            .get(&min_party)
                            .unwrap_or(&HashMap::new())
                            .get(p)
                            .cloned()
                            .unwrap_or(0.0)
                    })
                    .sum();

                for party in &remaining_parties {
                    if let Some(&party_votes) = preference_flows
                        .get(&min_party)
                        .unwrap_or(&HashMap::new())
                        .get(party)
                    {
                        let proportional_share =
                            (party_votes / total_remaining_percentage) * redistributed_votes as f32;
                        *votes.get_mut(party).unwrap() += proportional_share.round() as u32;
                    }
                }
            }
        }

        // Determine the winner among the last two remaining parties
        let (winner, _) = votes.iter().max_by_key(|&(_, &votes)| votes).unwrap();
        *seat_wins.entry(winner.clone()).or_insert(0) += 1;
    }

    return seat_wins
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
        }
        1 => {
            simulate_an_election();
        }
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
    let electoral_systems = &[
        "First Past The Post",
        "Proportional Representation",
        "Alternative Vote",
    ];
    let system_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an electoral system to simulate results")
        .default(0)
        .items(&electoral_systems[..])
        .interact()
        .unwrap();

    let electoral_system = match system_selection {
        0 => ElectoralSystem::FirstPastThePost,
        1 => ElectoralSystem::ProportionalRepresentation,
        2 => ElectoralSystem::AlternativeVote,
        _ => unreachable!(),
    };

     if let ElectoralSystem::AlternativeVote = electoral_system {
        let preference_flows_file = select_preference_flows_file();
        let preference_flows = load_preference_flows(&preference_flows_file);
        let simulated_result = simulate_election(&election_result, &electoral_system, Some(preference_flows));
        println!(
            "Simulated result: {:?}", simulated_result);    
    } else {
        let simulated_result = simulate_election(&election_result, &electoral_system, None);
        println!(
            "Simulated result: {:?}", simulated_result);    
    };

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
            let electoral_systems = &[
                "First Past The Post",
                "Alternative Vote",
            ];
            let system_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose an electoral system to simulate results")
                .default(0)
                .items(&electoral_systems[..])
                .interact()
                .unwrap();

            // let electoral_system = match system_selection {
            //     0 => ElectoralSystem::FirstPastThePost,
            //     1 => ElectoralSystem::AlternativeVote,
            //     _ => unreachable!(),
            // };

            // let simulated_result = if let ElectoralSystem::AlternativeVote = electoral_system {
            //     let preference_flows_file = select_preference_flows_file();
            //     let preference_flows = load_preference_flows(&preference_flows_file);
            //         simulate_election(&election_result, &electoral_system, Some(preference_flows))
            // } else {
            //     simulate_election(&election_result, &electoral_system, None)
            // };
            match system_selection {
                0 => {
                    let election_result = setup_two_party_fptp_election();
                    let simulated_result = simulate_election(&election_result, &ElectoralSystem::FirstPastThePost, None);
                    println!("Simulated Result: {:?}", simulated_result);
                }
                1 => {
                    let election_result = setup_two_party_fptp_election();
                    let preference_flows_file = select_preference_flows_file();
                    let preference_flows = load_preference_flows(&preference_flows_file);
                    let simulated_result = simulate_election(&election_result, &ElectoralSystem::AlternativeVote, Some(preference_flows));
                    println!("Simulated Result: {:?}", simulated_result);
                },
                _ => unreachable!(),       
            }
        }
        _ => unreachable!(),
    }
}

fn select_preference_flows_file() -> String {
    let options = &["preference_flows_england.json"]; // Add more files as needed
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a preference flow file")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    options[selection].to_string()
}

fn load_preference_flows(file_name: &str) -> HashMap<String, HashMap<String, f32>> {
    let file_path = format!("data/{}", file_name); // Adjust the path as necessary
    let file_content = fs::read_to_string(&file_path).expect("Unable to read file");
    serde_json::from_str(&file_content).expect("JSON was not well-formatted")
}

fn setup_two_party_fptp_election() -> ElectionResult {
    let party1 = Party {
        name: String::from("Party A"),
    };
    let party2 = Party {
        name: String::from("Party B"),
    };

    let candidate1 = Candidate {
        name: String::from("Alice"),
        party: party1.clone(),
    };
    let candidate2 = Candidate {
        name: String::from("Bob"),
        party: party2.clone(),
    };

    let constituency = Constituency {
        name: String::from("Constituency 1"),
        candidates: vec![candidate1, candidate2],
    };

    let constituency_result = ConstituencyResult {
        constituency: constituency.clone(),
        results: [(party1.name.clone(), 2), (party2.name.clone(), 1)]
            .iter()
            .cloned()
            .collect(),
    };

    ElectionResult {
        datetime: Utc::now(),
        constituencies: vec![constituency_result],
        overall_result: [(party1.name.clone(), 2), (party2.name.clone(), 1)]
            .iter()
            .cloned()
            .collect(),
    }
}

fn get_preference_flows(election_result: &ElectionResult) -> HashMap<String, HashMap<String, f32>> {
    let mut preference_flows = HashMap::new();

    for constituency_result in &election_result.constituencies {
        for candidate in &constituency_result.constituency.candidates {
            if !preference_flows.contains_key(&candidate.party.name) {
                let mut flow = HashMap::new();
                for other_candidate in &constituency_result.constituency.candidates {
                    if candidate.party.name != other_candidate.party.name {
                        flow.insert(other_candidate.party.name.clone(), 0.5);
                    }
                }
                preference_flows.insert(candidate.party.name.clone(), flow);
            }
        }
    }

    preference_flows
}
