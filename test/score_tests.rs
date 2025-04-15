// tests/core/score_tests.rs

use hermes_core::score_decay;
use hermes_core::score_boost;

#[test]
fn test_score_decay() {
    let mut score = 100.0;
    let initial_score = score;
    score_decay::apply_score_decay(&mut score, 0, 100, 0.01);
    assert!(score < initial_score); // Ensure score decayed
}

#[test]
fn test_score_boost() {
    let mut score = 50.0;
    let initial_score = score;
    score_boost::apply_score_boost(&mut score, 10.0, 0.1);
    assert!(score > initial_score); // Ensure score boosted
}

