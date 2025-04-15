// test/core/signal_buffer_tests.rs

use hermes_core::swarm::signal_buffer::{BufferedSignal, SignalBuffer};

#[test]
fn test_signal_is_not_duplicate_if_not_inserted() {
    let buffer = SignalBuffer::new();
    // Test that a signal that is not in the buffer is not considered a duplicate
    assert!(!buffer.is_duplicate("sig1", 100));
}

#[test]
fn test_signal_detected_as_duplicate() {
    let mut buffer = SignalBuffer::new();
    let sig = BufferedSignal {
        signal_id: "sig1".to_string(),
        timestamp: 100,
        ttl_seconds: 60,
    };
    buffer.insert(sig);
    // Test that a signal that is in the buffer and not expired is considered a duplicate
    assert!(buffer.is_duplicate("sig1", 120));
}

#[test]
fn test_signal_expires_properly() {
    let mut buffer = SignalBuffer::new();
    let sig = BufferedSignal {
        signal_id: "sig1".to_string(),
        timestamp: 100,
        ttl_seconds: 10,
    };
    buffer.insert(sig);
    // Test that the signal expires after TTL
    assert!(!buffer.is_duplicate("sig1", 111)); // Should not be duplicate at 111
    assert!(!buffer.is_duplicate("sig1", 200)); // Should be expired after TTL
}

#[test]
fn test_prune_removes_expired() {
    let mut buffer = SignalBuffer::new();
    // Insert two signals, one expired and one not
    buffer.insert(BufferedSignal {
        signal_id: "a".to_string(),
        timestamp: 100,
        ttl_seconds: 10,
    });
    buffer.insert(BufferedSignal {
        signal_id: "b".to_string(),
        timestamp: 120,
        ttl_seconds: 30,
    });

    // Prune expired signals at time 150
    buffer.prune(150);

    // Only signal "b" should remain
    assert_eq!(buffer.buffer.len(), 1);
    assert!(buffer.buffer.contains_key("b"));
    assert!(!buffer.buffer.contains_key("a"));
}

#[test]
fn test_active_count_only_counts_valid() {
    let mut buffer = SignalBuffer::new();
    // Insert signal 'a' with TTL of 20 seconds (expires at 120)
    buffer.insert(BufferedSignal {
        signal_id: "a".to_string(),
        timestamp: 100,
        ttl_seconds: 20,
    });
    // Insert signal 'b' with TTL of 20 seconds (expires at 150)
    buffer.insert(BufferedSignal {
        signal_id: "b".to_string(),
        timestamp: 130,
        ttl_seconds: 20,
    });

    // At time 115, only signal 'a' should be active
    assert_eq!(buffer.active_count(115), 1);

    // At time 160, both signals should be expired
    assert_eq!(buffer.active_count(160), 0);
}
