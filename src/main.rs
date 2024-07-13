use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ElectionResult {
    pub datetime: DateTime<Utc>,
    pub constituencies: Vec<ConstituencyResult>,
    pub overall_result: HashMap<Party, u32>, // Overall result by party
}

#[derive(Debug, Clone)]
pub struct ConstituencyResult {
    pub constituency: Constituency,
    pub results: HashMap<Party, u32>, // Results by party
}

#[derive(Debug, Clone)]
pub struct Constituency {
    pub name: String,
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Party {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Candidate {
    pub name: String,
    pub party: Party,
}

#[derive(Debug, Clone)]
pub enum ElectoralSystem {
    FirstPastThePost,
    // Add more systems here
}

pub fn simulate_election(election_result: &ElectionResult, electoral_system: &ElectoralSystem) -> HashMap<Party, u32> {
    match electoral_system {
        ElectoralSystem::FirstPastThePost => simulate_first_past_the_post(election_result),
        //ElectoralSystem::ProportionalRepresentation => simulate_proportional_representation(election_result),
        // Add more systems here
    }
}

fn simulate_first_past_the_post(election_result: &ElectionResult) -> HashMap<Party, u32> {
    let mut seat_wins: HashMap<Party, u32> = HashMap::new();
    
    for constituency_result in &election_result.constituencies {
        if let Some((winning_party, _)) = constituency_result.results.iter().max_by_key(|&(_, votes)| votes) {
            *seat_wins.entry(winning_party.clone()).or_insert(0) += 1;
        }
    }
    
    seat_wins
}

//fn simulate_proportional_representation(_election_result: &ElectionResult) -> HashMap<Party, u32> {
    // Implement Proportional Representation logic here
  //  HashMap::new()
//}

fn main() {
    // Example usage
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
        results: [(party1.clone(), 1000), (party2.clone(), 800)].iter().cloned().collect(),
    };

    let election_result = ElectionResult {
        datetime: Utc::now(),
        constituencies: vec![constituency_result],
        overall_result: [(party1, 1000), (party2, 800)].iter().cloned().collect(),
    };

    let electoral_system = ElectoralSystem::FirstPastThePost;

    // Simulate election under the chosen system
    let simulated_result = simulate_election(&election_result, &electoral_system);
    println!("{:?}", simulated_result);
}
