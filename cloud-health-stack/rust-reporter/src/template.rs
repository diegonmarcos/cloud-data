use std::collections::HashMap;

const TEMPLATE_PATH: &str = "container_health.md.tpl";
const OUTPUT_PATH: &str = "container_health.md";

/// Read template, replace $VARS (longest key first), write output
pub fn render(vars: &HashMap<String, String>) -> anyhow::Result<()> {
    let mut template = std::fs::read_to_string(TEMPLATE_PATH)?;
    println!("Template loaded ({} chars, {} vars)", template.len(), vars.len());

    // Sort keys longest first to prevent partial matches
    let mut sorted_keys: Vec<&String> = vars.keys().collect();
    sorted_keys.sort_by(|a, b| b.len().cmp(&a.len()));

    for key in sorted_keys {
        let placeholder = format!("${}", key);
        if template.contains(&placeholder) {
            template = template.replace(&placeholder, &vars[key]);
        } else {
            eprintln!("  WARN: ${} not found in template", key);
        }
    }

    // Check for unreplaced vars
    let unreplaced: Vec<&str> = template.match_indices('$')
        .filter_map(|(i, _)| {
            let rest = &template[i..];
            if rest.starts_with("${") || rest.starts_with("$POSTGRES") { return None; }
            let end = rest.find(|c: char| !c.is_ascii_uppercase() && c != '_').unwrap_or(rest.len());
            if end > 1 { Some(&rest[..end]) } else { None }
        })
        .collect();
    if !unreplaced.is_empty() {
        eprintln!("  WARN: unreplaced vars: {:?}", unreplaced);
    }

    std::fs::write(OUTPUT_PATH, template)?;
    println!("Wrote {}", OUTPUT_PATH);
    Ok(())
}
