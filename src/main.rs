use clap::Parser;
use colored::*;
use serde::Deserialize;
use std::fs;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(name = "tfplan")]
#[command(about = "Format Terraform plan output in human-readable format")]
struct Cli {
    /// Path to Terraform plan JSON file (use - for stdin)
    #[arg(value_name = "FILE")]
    file: Option<String>,

    /// Show collapsed view (only resource headers)
    #[arg(short, long)]
    collapsed: bool,

    /// Interactive mode - press number to expand/collapse resources
    #[arg(short, long)]
    interactive: bool,

    /// Output as HTML with expandable sections
    #[arg(long)]
    html: bool,
}

#[derive(Deserialize)]
struct TerraformPlan {
    resource_changes: Vec<ResourceChange>,
}

#[derive(Deserialize)]
struct ResourceChange {
    address: String,
    change: Change,
    r#type: String,
}

#[derive(Deserialize)]
struct Change {
    actions: Vec<String>,
    before: Option<serde_json::Value>,
    after: Option<serde_json::Value>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let content = match cli.file.as_deref() {
        Some("-") | None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
        Some(path) => fs::read_to_string(path)?,
    };

    let plan: TerraformPlan = serde_json::from_str(&content)?;

    if cli.html {
        html_format(&plan);
    } else if cli.interactive {
        interactive_format(&plan)?;
    } else {
        format_plan(&plan, cli.collapsed);
    }

    Ok(())
}

fn format_plan(plan: &TerraformPlan, collapsed: bool) {
    let mut create_count = 0;
    let mut update_count = 0;
    let mut delete_count = 0;
    let mut replace_count = 0;

    for change in &plan.resource_changes {
        match change.change.actions.as_slice() {
            [action] if action == "create" => {
                create_count += 1;
                print_resource_change(&change, "create", "green", collapsed);
            }
            [action] if action == "update" => {
                update_count += 1;
                print_resource_change(&change, "update", "yellow", collapsed);
            }
            [action] if action == "delete" => {
                delete_count += 1;
                print_resource_change(&change, "destroy", "red", collapsed);
            }
            [action1, action2] if action1 == "delete" && action2 == "create" => {
                replace_count += 1;
                print_resource_change(&change, "replace", "yellow", collapsed);
            }
            _ => {}
        }
    }

    println!();
    println!(
        "{}",
        "────────────────────────────────────────────────────────────────────────────────"
            .bright_black()
    );
    println!();

    let total_changes = create_count + update_count + replace_count + delete_count;
    if total_changes == 0 {
        println!(
            "{}",
            "No changes. Your infrastructure matches the configuration.".bright_green()
        );
    } else {
        println!(
            "{}: {} to add, {} to change, {} to destroy.",
            "Plan".bold(),
            create_count.to_string().bright_green(),
            (update_count + replace_count).to_string().bright_yellow(),
            delete_count.to_string().bright_red()
        );
    }
}

fn print_resource_change(change: &ResourceChange, action: &str, color: &str, collapsed: bool) {
    let symbol = match action {
        "create" => "+",
        "update" => "~",
        "destroy" => "-",
        "replace" => "-/+",
        _ => "?",
    };

    let colored_symbol = match color {
        "green" => symbol.bright_green().bold(),
        "yellow" => symbol.bright_yellow().bold(),
        "red" => symbol.bright_red().bold(),
        _ => symbol.normal(),
    };

    let action_text = match action {
        "create" => "created".bright_green(),
        "update" => "changed".bright_yellow(),
        "destroy" => "destroyed".bright_red(),
        "replace" => "replaced".bright_yellow(),
        _ => action.normal(),
    };

    let expand_indicator = if collapsed { "▶" } else { "▼" };

    println!(
        "{} {} {} will be {}",
        expand_indicator.bright_black(),
        colored_symbol,
        change.address.bold(),
        action_text
    );

    if !collapsed {
        if action == "update" || action == "replace" {
            print_changes(&change.change);
        } else if action == "create" {
            print_create_attributes(&change.change);
        }
    }

    println!();
}

fn print_changes(change: &Change) {
    if let (Some(before), Some(after)) = (&change.before, &change.after) {
        if let (Some(before_obj), Some(after_obj)) = (before.as_object(), after.as_object()) {
            for (key, after_val) in after_obj {
                if let Some(before_val) = before_obj.get(key) {
                    if before_val != after_val {
                        println!(
                            "        {}: {} {} {}",
                            key.bright_white(),
                            format_value(before_val).bright_red(),
                            "=>".bright_black(),
                            format_value(after_val).bright_green()
                        );
                    }
                } else {
                    println!(
                        "        {}: {}",
                        key.bright_white(),
                        format_value(after_val).bright_green()
                    );
                }
            }
            for (key, before_val) in before_obj {
                if !after_obj.contains_key(key) {
                    println!(
                        "        {}: {} {} {}",
                        key.bright_white(),
                        format_value(before_val).bright_red(),
                        "=>".bright_black(),
                        "null".bright_red()
                    );
                }
            }
        }
    }
}

fn print_create_attributes(change: &Change) {
    if let Some(after) = &change.after {
        if let Some(after_obj) = after.as_object() {
            for (key, val) in after_obj {
                println!(
                    "        {}: {}",
                    key.bright_white(),
                    format_value(val).bright_green()
                );
            }
        }
    }
}

fn interactive_format(plan: &TerraformPlan) -> Result<(), Box<dyn std::error::Error>> {
    let mut expanded: std::collections::HashSet<usize> = std::collections::HashSet::new();

    loop {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen

        println!("Interactive Plan (Enter number to toggle, 'a' for all, 'c' to collapse all, 'q' to quit):");
        println!();

        for (i, change) in plan.resource_changes.iter().enumerate() {
            let is_expanded = expanded.contains(&i);
            print_interactive_resource(i, change, is_expanded);
        }

        print!("\nCommand: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "q" => break,
            "a" => {
                for i in 0..plan.resource_changes.len() {
                    expanded.insert(i);
                }
            }
            "c" => expanded.clear(),
            n => {
                if let Ok(idx) = n.parse::<usize>() {
                    if idx < plan.resource_changes.len() {
                        if expanded.contains(&idx) {
                            expanded.remove(&idx);
                        } else {
                            expanded.insert(idx);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn print_interactive_resource(index: usize, change: &ResourceChange, is_expanded: bool) {
    let (action, color) = match change.change.actions.as_slice() {
        [a] if a == "create" => ("created", "green"),
        [a] if a == "update" => ("changed", "yellow"),
        [a] if a == "delete" => ("destroyed", "red"),
        [a1, a2] if a1 == "delete" && a2 == "create" => ("replaced", "yellow"),
        _ => ("unknown", "white"),
    };

    let symbol = match action {
        "created" => "+".bright_green().bold(),
        "changed" => "~".bright_yellow().bold(),
        "destroyed" => "-".bright_red().bold(),
        "replaced" => "-/+".bright_yellow().bold(),
        _ => "?".normal(),
    };

    let indicator = if is_expanded { "▼" } else { "▶" };

    println!(
        "{} {} {} {} will be {}",
        format!("[{}]", index).bright_cyan(),
        indicator.bright_black(),
        symbol,
        change.address.bold(),
        action.color(color)
    );

    if is_expanded {
        match action {
            "changed" | "replaced" => print_changes(&change.change),
            "created" => print_create_attributes(&change.change),
            _ => {}
        }
    }
    println!();
}

fn html_format(plan: &TerraformPlan) {
    println!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Terraform Plan</title>
    <style>
        body {{ font-family: 'Monaco', 'Menlo', monospace; background: #1e1e1e; color: #d4d4d4; padding: 20px; }}
        .resource {{ margin: 10px 0; }}
        .resource-header {{ cursor: pointer; padding: 8px; border-radius: 4px; background: #2d2d30; }}
        .resource-header:hover {{ background: #3e3e42; }}
        .create {{ color: #4ec9b0; }}
        .update {{ color: #dcdcaa; }}
        .destroy {{ color: #f44747; }}
        .replace {{ color: #dcdcaa; }}
        .details {{ margin-left: 20px; padding: 10px; background: #252526; border-radius: 4px; display: none; }}
        .attribute {{ margin: 4px 0; }}
        .key {{ color: #9cdcfe; }}
        .value-old {{ color: #f44747; }}
        .value-new {{ color: #4ec9b0; }}
        .arrow {{ color: #808080; }}
        .summary {{ margin-top: 20px; padding: 15px; background: #2d2d30; border-radius: 4px; border-top: 3px solid #007acc; }}
        .expand-icon {{ display: inline-block; width: 12px; transition: transform 0.2s; }}
        .expanded .expand-icon {{ transform: rotate(90deg); }}
    </style>
</head>
<body>
    <h1>Terraform Plan</h1>
"#
    );

    let mut create_count = 0;
    let mut update_count = 0;
    let mut delete_count = 0;
    let mut replace_count = 0;

    for (i, change) in plan.resource_changes.iter().enumerate() {
        match change.change.actions.as_slice() {
            [action] if action == "create" => {
                create_count += 1;
                print_html_resource(i, &change, "create", "create");
            }
            [action] if action == "update" => {
                update_count += 1;
                print_html_resource(i, &change, "update", "update");
            }
            [action] if action == "delete" => {
                delete_count += 1;
                print_html_resource(i, &change, "destroy", "destroy");
            }
            [action1, action2] if action1 == "delete" && action2 == "create" => {
                replace_count += 1;
                print_html_resource(i, &change, "replace", "replace");
            }
            _ => {}
        }
    }

    println!(
        r#"    <div class="summary">
        <h3>Plan Summary</h3>
        <p><span style="color: #4ec9b0;">{}</span> to add, <span style="color: #dcdcaa;">{}</span> to change, <span style="color: #f44747;">{}</span> to destroy.</p>
    </div>
"#,
        create_count,
        update_count + replace_count,
        delete_count
    );

    println!(
        r#"    <script>
        function toggleResource(id) {{
            const details = document.getElementById('details-' + id);
            const header = document.getElementById('header-' + id);
            if (details.style.display === 'none') {{
                details.style.display = 'block';
                header.classList.add('expanded');
            }} else {{
                details.style.display = 'none';
                header.classList.remove('expanded');
            }}
        }}
    </script>
</body>
</html>"#
    );
}

fn print_html_resource(index: usize, change: &ResourceChange, action: &str, css_class: &str) {
    let action_text = match action {
        "create" => "created",
        "update" => "changed",
        "destroy" => "destroyed",
        "replace" => "replaced",
        _ => action,
    };

    let symbol = match action {
        "create" => "+",
        "update" => "~",
        "destroy" => "-",
        "replace" => "-/+",
        _ => "?",
    };

    println!(
        r#"    <div class="resource">
        <div class="resource-header {}" id="header-{}" onclick="toggleResource({})">
            <span class="expand-icon">▶</span> {} <strong>{}</strong> will be {}
        </div>
        <div class="details" id="details-{}">
"#,
        css_class, index, index, symbol, change.address, action_text, index
    );

    if action == "update" || action == "replace" {
        print_html_changes(&change.change);
    } else if action == "create" {
        print_html_create_attributes(&change.change);
    }

    println!("        </div>");
    println!("    </div>");
}

fn print_html_changes(change: &Change) {
    if let (Some(before), Some(after)) = (&change.before, &change.after) {
        if let (Some(before_obj), Some(after_obj)) = (before.as_object(), after.as_object()) {
            for (key, after_val) in after_obj {
                if let Some(before_val) = before_obj.get(key) {
                    if before_val != after_val {
                        println!(
                            r#"            <div class="attribute">
                <span class="key">{}:</span> 
                <span class="value-old">{}</span> 
                <span class="arrow">=></span> 
                <span class="value-new">{}</span>
            </div>"#,
                            html_escape(key),
                            html_escape(&format_value(before_val)),
                            html_escape(&format_value(after_val))
                        );
                    }
                } else {
                    println!(
                        r#"            <div class="attribute">
                <span class="key">{}:</span> 
                <span class="value-new">{}</span>
            </div>"#,
                        html_escape(key),
                        html_escape(&format_value(after_val))
                    );
                }
            }
            for (key, before_val) in before_obj {
                if !after_obj.contains_key(key) {
                    println!(
                        r#"            <div class="attribute">
                <span class="key">{}:</span> 
                <span class="value-old">{}</span> 
                <span class="arrow">=></span> 
                <span class="value-old">null</span>
            </div>"#,
                        html_escape(key),
                        html_escape(&format_value(before_val))
                    );
                }
            }
        }
    }
}

fn print_html_create_attributes(change: &Change) {
    if let Some(after) = &change.after {
        if let Some(after_obj) = after.as_object() {
            for (key, val) in after_obj {
                println!(
                    r#"            <div class="attribute">
                <span class="key">{}:</span> 
                <span class="value-new">{}</span>
            </div>"#,
                    html_escape(key),
                    html_escape(&format_value(val))
                );
            }
        }
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn format_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => format!("\"{}\"", s),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => "null".to_string(),
        _ => value.to_string(),
    }
}
