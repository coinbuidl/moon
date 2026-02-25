use crate::moon::config::MoonConfig;
use crate::moon::session_usage::SessionUsageSnapshot;
use crate::moon::state::MoonState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerKind {
    Archive,
    Compaction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContextCompactionDecision {
    pub should_compact: bool,
    pub activate_hysteresis: bool,
    pub clear_hysteresis: bool,
    pub bypassed_cooldown: bool,
}

pub fn evaluate_context_compaction_candidate(
    usage_ratio: f64,
    start_ratio: f64,
    emergency_ratio: f64,
    recover_ratio: f64,
    cooldown_ready: bool,
    hysteresis_active: bool,
) -> ContextCompactionDecision {
    if hysteresis_active {
        if usage_ratio <= recover_ratio {
            return ContextCompactionDecision {
                should_compact: false,
                activate_hysteresis: false,
                clear_hysteresis: true,
                bypassed_cooldown: false,
            };
        }
        return ContextCompactionDecision {
            should_compact: false,
            activate_hysteresis: false,
            clear_hysteresis: false,
            bypassed_cooldown: false,
        };
    }

    if usage_ratio < start_ratio {
        return ContextCompactionDecision {
            should_compact: false,
            activate_hysteresis: false,
            clear_hysteresis: false,
            bypassed_cooldown: false,
        };
    }

    let bypassed_cooldown = !cooldown_ready && usage_ratio >= emergency_ratio;
    if cooldown_ready || bypassed_cooldown {
        return ContextCompactionDecision {
            should_compact: true,
            activate_hysteresis: true,
            clear_hysteresis: false,
            bypassed_cooldown,
        };
    }

    ContextCompactionDecision {
        should_compact: false,
        activate_hysteresis: false,
        clear_hysteresis: false,
        bypassed_cooldown: false,
    }
}

fn unified_layer1_last_trigger(state: &MoonState) -> Option<u64> {
    match (
        state.last_archive_trigger_epoch_secs,
        state.last_compaction_trigger_epoch_secs,
    ) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(v), None) | (None, Some(v)) => Some(v),
        (None, None) => None,
    }
}

fn should_fire(last_epoch: Option<u64>, now_epoch: u64, cooldown_secs: u64) -> bool {
    match last_epoch {
        None => true,
        Some(last) => now_epoch.saturating_sub(last) >= cooldown_secs,
    }
}

pub fn evaluate(
    cfg: &MoonConfig,
    state: &MoonState,
    usage: &SessionUsageSnapshot,
) -> Vec<TriggerKind> {
    let mut out = Vec::new();
    let now = usage.captured_at_epoch_secs;
    if usage.usage_ratio >= cfg.thresholds.trigger_ratio
        && should_fire(
            unified_layer1_last_trigger(state),
            now,
            cfg.watcher.cooldown_secs,
        )
    {
        // Unified trigger: archive-before-compact protocol.
        out.push(TriggerKind::Archive);
        out.push(TriggerKind::Compaction);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moon::config::MoonConfig;

    #[test]
    fn evaluate_respects_order_and_thresholds() {
        let cfg = MoonConfig::default();
        let state = MoonState::default();
        let usage = SessionUsageSnapshot {
            session_id: "s".into(),
            used_tokens: 95,
            max_tokens: 100,
            usage_ratio: 0.95,
            captured_at_epoch_secs: 1000,
            provider: "t".into(),
        };

        let triggers = evaluate(&cfg, &state, &usage);
        assert_eq!(
            triggers,
            vec![TriggerKind::Archive, TriggerKind::Compaction]
        );
    }

    #[test]
    fn evaluate_respects_unified_cooldown() {
        let cfg = MoonConfig::default();
        let state = MoonState::default();
        let usage = SessionUsageSnapshot {
            session_id: "s".into(),
            used_tokens: 95,
            max_tokens: 100,
            usage_ratio: 0.95,
            captured_at_epoch_secs: 1000,
            provider: "t".into(),
        };

        let triggers = evaluate(&cfg, &state, &usage);
        assert_eq!(
            triggers,
            vec![TriggerKind::Archive, TriggerKind::Compaction]
        );

        let mut state_in_cooldown = state.clone();
        state_in_cooldown.last_archive_trigger_epoch_secs = Some(995);
        state_in_cooldown.last_compaction_trigger_epoch_secs = Some(998);
        let triggers_cooldown = evaluate(&cfg, &state_in_cooldown, &usage);
        assert!(triggers_cooldown.is_empty());
    }

    #[test]
    fn context_compaction_bypasses_cooldown_only_on_emergency() {
        let start = 0.78;
        let emergency = 0.90;
        let recover = 0.65;

        let regular =
            evaluate_context_compaction_candidate(0.85, start, emergency, recover, false, false);
        assert!(!regular.should_compact);
        assert!(!regular.bypassed_cooldown);

        let emergency_hit =
            evaluate_context_compaction_candidate(0.95, start, emergency, recover, false, false);
        assert!(emergency_hit.should_compact);
        assert!(emergency_hit.activate_hysteresis);
        assert!(emergency_hit.bypassed_cooldown);
    }

    #[test]
    fn context_compaction_hysteresis_blocks_until_recover() {
        let start = 0.78;
        let emergency = 0.90;
        let recover = 0.65;

        let blocked =
            evaluate_context_compaction_candidate(0.82, start, emergency, recover, true, true);
        assert!(!blocked.should_compact);
        assert!(!blocked.clear_hysteresis);

        let clear =
            evaluate_context_compaction_candidate(0.60, start, emergency, recover, true, true);
        assert!(!clear.should_compact);
        assert!(clear.clear_hysteresis);
    }
}
