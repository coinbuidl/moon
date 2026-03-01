use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::commands::CommandReport;
use crate::moon::archive::{ArchiveRecord, projection_path_for_archive, read_ledger_records};
use crate::moon::distill::{
    DistillInput, WisdomDistillInput, archive_file_size, run_distillation, run_wisdom_distillation,
};
use crate::moon::paths::{MoonPaths, resolve_paths};
use crate::moon::state::load;

#[derive(Debug, Clone)]
pub struct MoonDistillOptions {
    pub mode: String,
    pub archive_path: Option<String>,
    pub files: Vec<String>,
    pub session_id: Option<String>,
    pub dry_run: bool,
}

fn is_distillable_archive_record(record: &ArchiveRecord) -> bool {
    let source_path = Path::new(&record.source_path);
    let archive_path = Path::new(&record.archive_path);

    let source_file = source_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let archive_file = archive_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let source_ext = source_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let archive_ext = archive_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    if source_file == "sessions.json" {
        return false;
    }
    if source_ext == "lock"
        || archive_ext == "lock"
        || source_file.ends_with(".lock")
        || archive_file.ends_with(".lock")
    {
        return false;
    }
    if archive_ext == "json" && archive_file.starts_with("sessions-") {
        return false;
    }

    true
}

fn resolve_norm_projection_path(paths: &MoonPaths, record: &ArchiveRecord) -> Option<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(path) = record.projection_path.as_deref() {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            candidates.push(PathBuf::from(trimmed));
        }
    }
    candidates.push(projection_path_for_archive(&record.archive_path));

    for candidate in candidates {
        if !candidate.exists() {
            continue;
        }
        let is_markdown = candidate
            .extension()
            .and_then(|v| v.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"));
        if !is_markdown {
            continue;
        }
        let mlib_root = paths.archives_dir.join("mlib");
        let normalized_candidate =
            fs::canonicalize(&candidate).unwrap_or_else(|_| candidate.clone());
        let normalized_mlib_root = fs::canonicalize(&mlib_root).unwrap_or(mlib_root);
        let in_mlib = normalized_candidate.starts_with(normalized_mlib_root);
        if in_mlib {
            return Some(candidate);
        }
    }

    None
}

fn normalize_path(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn resolve_pending_manual_norm_target(
    paths: &MoonPaths,
    archive_path: &Path,
) -> Result<(ArchiveRecord, String)> {
    let state = load(paths)?;
    let requested = normalize_path(archive_path);

    let mut matched: Option<(ArchiveRecord, String)> = None;
    for record in read_ledger_records(paths)? {
        if !record.indexed || state.distilled_archives.contains_key(&record.archive_path) {
            continue;
        }
        if !is_distillable_archive_record(&record) {
            continue;
        }
        let Some(projection_path) = resolve_norm_projection_path(paths, &record) else {
            continue;
        };
        if normalize_path(&projection_path) != requested {
            continue;
        }

        let projection_display = projection_path.display().to_string();
        match &matched {
            Some((current, _)) if current.created_at_epoch_secs <= record.created_at_epoch_secs => {
            }
            _ => matched = Some((record, projection_display)),
        }
    }

    match matched {
        Some(found) => Ok(found),
        None => {
            anyhow::bail!(
                "norm source is not pending: no matching undistilled archives/mlib/*.md found in ledger"
            )
        }
    }
}

pub fn run(opts: &MoonDistillOptions) -> Result<CommandReport> {
    let paths = resolve_paths()?;
    let mut report = CommandReport::new("distill");

    let mode = opts.mode.trim().to_ascii_lowercase();
    let normalized_mode = match mode.as_str() {
        "norm" | "l1" | "layer1" | "l1-normalisation" | "l1-normalization" | "" => "norm",
        "syns" | "syn" | "wisdom" | "layer2" | "l2-synthesis" | "l2-distillation" => "syns",
        _ => {
            report.issue(format!(
                "invalid distill mode `{}`; use `norm` or `syns`",
                opts.mode
            ));
            return Ok(report);
        }
    };

    if normalized_mode == "syns" {
        if opts.dry_run {
            report.detail("distill.dry_run=true".to_string());
        }
        let out = match run_wisdom_distillation(
            &paths,
            &WisdomDistillInput {
                trigger: "manual-distill".to_string(),
                day_epoch_secs: None,
                source_paths: opts.files.clone(),
                dry_run: opts.dry_run,
            },
        ) {
            Ok(out) => out,
            Err(err) => {
                let err_text = format!("{err:#}");
                report.issue(format!("syns skipped: {err_text}"));
                let lower = err_text.to_ascii_lowercase();
                if lower.contains("moon_wisdom_provider")
                    || lower.contains("moon_wisdom_model")
                    || lower.contains("primary model")
                    || lower.contains("provider credentials")
                    || lower.contains("api key")
                {
                    report.issue(
                        "fix MOON_WISDOM_PROVIDER, MOON_WISDOM_MODEL, and provider API key"
                            .to_string(),
                    );
                }
                return Ok(report);
            }
        };
        report.detail("distill.mode=syns".to_string());
        report.detail(format!("provider={}", out.provider));
        report.detail(format!("summary_path={}", out.summary_path));
        report.detail(format!("audit_log_path={}", out.audit_log_path));
        return Ok(report);
    }

    let archive_path = match opts.archive_path.as_deref() {
        Some(path) if !path.trim().is_empty() => path,
        _ => {
            report.issue("archive path cannot be empty in norm mode");
            return Ok(report);
        }
    };

    let archive_file = Path::new(archive_path);
    let is_projection_md = archive_file
        .extension()
        .and_then(|v| v.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"));
    if !is_projection_md {
        anyhow::bail!("norm mode requires -archive <archives/mlib/*.md>");
    }
    if !archive_file.is_file() {
        anyhow::bail!("norm archive path is not a readable file: {}", archive_path);
    }
    let _ = fs::File::open(archive_file)
        .with_context(|| format!("failed to open norm archive {}", archive_path))?;
    let archive_size = archive_file_size(archive_path)
        .with_context(|| format!("failed to stat {}", archive_path))?;

    let (pending_record, pending_projection_path) =
        resolve_pending_manual_norm_target(&paths, archive_file)?;
    let session_id = opts
        .session_id
        .clone()
        .unwrap_or_else(|| pending_record.session_id.clone());
    let archive_epoch_secs = Some(pending_record.created_at_epoch_secs);

    if opts.dry_run {
        report.detail("distill.dry_run=true".to_string());
        report.detail(format!("archive_size_bytes={archive_size}"));
        report.detail("distill.mode=norm".to_string());
        return Ok(report);
    }

    let out = run_distillation(
        &paths,
        &DistillInput {
            session_id,
            archive_path: pending_projection_path,
            archive_text: String::new(),
            archive_epoch_secs,
        },
    )?;
    report.detail("distill.mode=norm".to_string());

    report.detail(format!("provider={}", out.provider));
    report.detail(format!("summary_path={}", out.summary_path));
    report.detail(format!("audit_log_path={}", out.audit_log_path));
    report.detail(format!("archive_size_bytes={archive_size}"));

    Ok(report)
}
