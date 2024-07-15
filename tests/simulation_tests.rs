use chrono::Utc;
use std::collections::HashMap;

#[path = "../src/main.rs"]
mod main;

use main::{
    simulate_election, Candidate, Constituency, ConstituencyResult, ElectionResult,
    ElectoralSystem, Party,
};

#[test]
fn test_first_past_the_post_simulation() {
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
        results: [(party1.clone(), 2), (party2.clone(), 1)]
            .iter()
            .cloned()
            .collect(),
    };

    let election_result = ElectionResult {
        datetime: Utc::now(),
        constituencies: vec![constituency_result],
        overall_result: [(party1.clone(), 2), (party2.clone(), 1)]
            .iter()
            .cloned()
            .collect(),
    };

    let electoral_system = ElectoralSystem::FirstPastThePost;

    let simulated_result = simulate_election(&election_result, &electoral_system);

    let expected_result: HashMap<Party, u32> = [(party1, 1)].iter().cloned().collect();
    assert_eq!(simulated_result, expected_result);
}
