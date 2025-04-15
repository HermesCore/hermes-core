// tests/core/agent_tests.rs

use hermes_core::agents::{ScoutAgent, AgentClass};
use hermes_core::score_decay;
use hermes_core::score_boost;

#[test]
fn test_scout_agent_initialization() {
    let scout = ScoutAgent::new("Scout1".to_string());
    assert_eq!(scout.get_score(), 50.0); // Default starting score
}

#[test]
fn test_score_decay() {
    let mut agent = ScoutAgent::new("Scout1".to_string());
    let initial_score = agent.get_score();
    score_decay::apply_score_decay(&mut agent.score, 0, 100, 0.01);
    assert!(agent.get_score() < initial_score); // Ensure score decayed
}

#[test]
fn test_score_boost() {
    let mut agent = ScoutAgent::new("Scout1".to_string());
    let initial_score = agent.get_score();
    score_boost::apply_score_boost(&mut agent.score, 10.0, 0.1);
    assert!(agent.get_score() > initial_score); // Ensure score boosted
}

#[test]
fn test_agent_execution() {
    let mut agent = ScoutAgent::new("Scout1".to_string());
    let initial_score = agent.get_score();
    let current_time = 100;
    agent.perform_action(current_time, 50.0);
    assert!(agent.get_score() > initial_score); // Ensure score boosted
}
