use anyhow::Result;

use crate::openclaw::gateway;

pub fn run_full_doctor() -> Result<()> {
    gateway::run_doctor()
}
