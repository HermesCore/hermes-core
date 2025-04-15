// scaling_trigger.rs
use crate::core::vault_growth_scaler::VaultGrowthScaler;

#[derive(Debug, PartialEq)]
pub enum ScalingDecision {
    Grow(usize),
    Shrink(usize),
    Hold,
}

pub struct ScalingTrigger {
    pub grow_threshold: f64,
    pub shrink_threshold: f64,
}

impl ScalingTrigger {
    pub fn new(grow_threshold: f64, shrink_threshold: f64) -> Self {
        Self { grow_threshold, shrink_threshold }
    }

    pub fn evaluate(&self, scaler: &VaultGrowthScaler) -> ScalingDecision {
        let (_, percent_growth) = scaler.compute_swarm_growth();
        if percent_growth > self.grow_threshold {
            println!("📈 Growth {:.2}% > threshold → GROW", percent_growth);
            ScalingDecision::Grow(5)
        } else if percent_growth < self.shrink_threshold {
            println!("📉 Growth {:.2}% < threshold → SHRINK", percent_growth);
            ScalingDecision::Shrink(5)
        } else {
            println!("⚖️  Growth {:.2}% → HOLD", percent_growth);
            ScalingDecision::Hold
        }
    }
}