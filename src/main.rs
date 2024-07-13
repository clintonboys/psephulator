// Load election results or simulate election
// Simulate results under chosen electoral system
use chrono::prelude::*;

pub struct ElectionResults{
    overall_result: OverallElectionResult,
    results_by_constituency: Map
}

pub struct Election{
    date: DateTime<Utc>,
    name: String,
    results: ElectionResults,
}

fn main() {
    println!("Hello, world!");
}
