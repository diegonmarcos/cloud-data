//! Z-Appendix builder.
//!
//! Daily absorbs two sub-engines (`health_full2`, `mail_full`) and surfaces
//! their reports as Z-sections (Z.1, Z.2, Z.3, …) so the consolidated report
//! exposes every check the former separate crates ran.
//!
//! Two entry points:
//!   - `from_reports(...)` — build from in-process submodule results (preferred).
//!   - `load()` — fall-back: read artefacts from cwd (standalone / partial pipeline).

use crate::health_full2::FullReport;
use crate::mail_full::MailReport;
use serde_json::Value;
use std::path::Path;

/// One Z-section in the appendix.
#[derive(Debug, Clone)]
pub struct ZSection {
    /// e.g. "Z.1", "Z.2".
    pub number: String,
    /// Human title — "Full 11-Layer Diagnostic", "Mail Health 7-Phase", …
    pub title: String,
    /// Origin of the content (submodule name).
    pub source: String,
    /// Rendered markdown body.
    pub markdown: String,
}

pub struct Appendix {
    pub sections: Vec<ZSection>,
    pub stack: Option<Value>,
    pub full: Option<Value>,
    pub mail: Option<Value>,
}

impl Appendix {
    pub fn is_empty(&self) -> bool {
        self.sections.iter().all(|s| s.markdown.trim().is_empty())
    }

    pub fn summary(&self) -> String {
        format!(
            "sections=[{}] stack={} full={} mail={}",
            self.sections
                .iter()
                .map(|s| format!("{}:{}L", s.number, s.markdown.lines().count()))
                .collect::<Vec<_>>()
                .join(","),
            if self.stack.is_some() { "yes" } else { "no" },
            if self.full.is_some() { "yes" } else { "no" },
            if self.mail.is_some() { "yes" } else { "no" },
        )
    }

    /// Flatten all sections into a single markdown blob with Z.N headers.
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        for s in &self.sections {
            out.push_str(&format!("\n## {} — {}\n\n", s.number, s.title));
            out.push_str(&format!("_Source: `{}`_\n\n", s.source));
            out.push_str(&s.markdown);
            out.push_str("\n\n---\n");
        }
        out
    }

    /// Single legacy `appendix_md` string — kept so existing templates that
    /// use `$APPENDIX` continue to work.
    pub fn legacy_md(&self) -> String {
        self.to_markdown()
    }
}

/// Preferred path: build appendix from in-process submodule results.
pub fn from_reports(full: Option<&FullReport>, mail: Option<&MailReport>) -> Appendix {
    let mut sections = Vec::new();

    if let Some(f) = full {
        sections.push(ZSection {
            number: "Z.1".into(),
            title: "11-Layer Diagnostic + Stack Topology".into(),
            source: "health_full2 (absorbed from cloud-health-full-2)".into(),
            markdown: f.markdown.clone(),
        });
    }
    if let Some(m) = mail {
        sections.push(ZSection {
            number: "Z.2".into(),
            title: "Mail Health 7-Phase Diagnostic".into(),
            source: "mail_full (absorbed from cloud-mail-health-full)".into(),
            markdown: m.markdown.clone(),
        });
    }

    Appendix {
        sections,
        stack: full.and_then(|f| f.stack.clone()),
        full: full.and_then(|f| serde_json::to_value(&f.results).ok()),
        mail: mail.and_then(|m| serde_json::to_value(&m.results).ok()),
    }
}

/// Fall-back: read appendix content from cwd (when sub-reports ran outside
/// this process). Graceful: missing files produce an empty appendix.
pub fn load() -> Appendix {
    let mut sections = Vec::new();

    let full_md = read_text("cloud_health_full.md");
    if !full_md.trim().is_empty() {
        sections.push(ZSection {
            number: "Z.1".into(),
            title: "11-Layer Diagnostic + Stack Topology".into(),
            source: "cloud_health_full.md".into(),
            markdown: full_md,
        });
    }
    let mail_md = read_text("cloud_mail_full.md");
    if !mail_md.trim().is_empty() {
        sections.push(ZSection {
            number: "Z.2".into(),
            title: "Mail Health 7-Phase Diagnostic".into(),
            source: "cloud_mail_full.md".into(),
            markdown: mail_md,
        });
    }

    Appendix {
        sections,
        stack: read_json("cloud_stack.json"),
        full: read_json("cloud_health_full.json"),
        mail: read_json("cloud_mail_full.json"),
    }
}

fn read_text<P: AsRef<Path>>(path: P) -> String {
    std::fs::read_to_string(path.as_ref()).unwrap_or_default()
}

fn read_json<P: AsRef<Path>>(path: P) -> Option<Value> {
    let raw = std::fs::read_to_string(path.as_ref()).ok()?;
    serde_json::from_str(&raw).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_appendix_serialises_cleanly() {
        let a = Appendix {
            sections: Vec::new(),
            stack: None,
            full: None,
            mail: None,
        };
        assert!(a.is_empty());
        assert_eq!(a.to_markdown(), "");
    }

    #[test]
    fn load_from_missing_cwd_files_yields_empty() {
        let tmp = std::env::temp_dir().join(format!("apx-test-{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        let prev = std::env::current_dir().unwrap();
        std::env::set_current_dir(&tmp).unwrap();
        let a = load();
        std::env::set_current_dir(prev).unwrap();
        let _ = std::fs::remove_dir_all(&tmp);
        assert!(a.is_empty(), "expected empty, got: {}", a.summary());
    }
}
