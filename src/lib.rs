
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct TerraformPlan {
    pub resource_changes: Vec<ResourceChange>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResourceChange {
    pub address: String,
    pub change: Change,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Change {
    pub actions: Vec<String>,
    pub before: Option<serde_json::Value>,
    pub after: Option<serde_json::Value>,
}

pub fn format_plan(plan: &TerraformPlan, collapsed: bool) -> String {
    let mut output = String::new();
    let mut counts = HashMap::new();

    for change in &plan.resource_changes {
        let action = get_action(&change.change.actions);
        *counts.entry(action).or_insert(0) += 1;
        output.push_str(&format_resource_change(change, collapsed));
    }

    output.push_str(&format_summary(&counts));
    output
}

fn get_action(actions: &[String]) -> &'static str {
    match actions {
        [action] if action == "create" => "create",
        [action] if action == "update" => "update", 
        [action] if action == "delete" => "delete",
        [a1, a2] if a1 == "delete" && a2 == "create" => "replace",
        _ => "unknown",
    }
}

fn format_resource_change(change: &ResourceChange, collapsed: bool) -> String {
    let action = get_action(&change.change.actions);
    let (symbol, _color) = match action {
        "create" => ("+", "green"),
        "update" => ("~", "yellow"),
        "delete" => ("-", "red"),
        "replace" => ("-/+", "yellow"),
        _ => ("?", "white"),
    };

    let indicator = if collapsed { "▶" } else { "▼" };
    let mut output = format!("{} {} {} will be {}\n", 
        indicator, symbol, change.address, action);

    if !collapsed {
        output.push_str(&format_changes(&change.change, action));
    }
    output.push('\n');
    output
}

fn format_changes(change: &Change, action: &str) -> String {
    let mut output = String::new();
    
    if action == "update" || action == "replace" {
        if let (Some(before), Some(after)) = (&change.before, &change.after) {
            if let (Some(before_obj), Some(after_obj)) = (before.as_object(), after.as_object()) {
                for (key, after_val) in after_obj {
                    if let Some(before_val) = before_obj.get(key) {
                        if before_val != after_val {
                            output.push_str(&format!("        {}: {} => {}\n", 
                                key, format_value(before_val), format_value(after_val)));
                        }
                    } else {
                        output.push_str(&format!("        {}: {}\n", 
                            key, format_value(after_val)));
                    }
                }
            }
        }
    } else if action == "create" {
        if let Some(after) = &change.after {
            if let Some(after_obj) = after.as_object() {
                for (key, val) in after_obj {
                    output.push_str(&format!("        {}: {}\n", key, format_value(val)));
                }
            }
        }
    }
    output
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

fn format_summary(counts: &HashMap<&str, i32>) -> String {
    let create_count = counts.get("create").unwrap_or(&0);
    let update_count = counts.get("update").unwrap_or(&0);
    let replace_count = counts.get("replace").unwrap_or(&0);
    let delete_count = counts.get("delete").unwrap_or(&0);

    let total_changes = create_count + update_count + replace_count + delete_count;
    
    if total_changes == 0 {
        "No changes. Your infrastructure matches the configuration.\n".to_string()
    } else {
        format!("Plan: {} to add, {} to change, {} to destroy.\n",
            create_count,
            update_count + replace_count,
            delete_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_test_plan() -> TerraformPlan {
        TerraformPlan {
            resource_changes: vec![
                ResourceChange {
                    address: "aws_instance.web".to_string(),
                    change: Change {
                        actions: vec!["create".to_string()],
                        before: None,
                        after: Some(json!({
                            "ami": "ami-12345678",
                            "instance_type": "t3.micro"
                        })),
                    },
                },
                ResourceChange {
                    address: "aws_s3_bucket.data".to_string(),
                    change: Change {
                        actions: vec!["update".to_string()],
                        before: Some(json!({
                            "encryption": null,
                            "versioning": false
                        })),
                        after: Some(json!({
                            "encryption": "AES256",
                            "versioning": true
                        })),
                    },
                },
            ],
        }
    }

    #[test]
    fn test_get_action() {
        assert_eq!(get_action(&["create".to_string()]), "create");
        assert_eq!(get_action(&["update".to_string()]), "update");
        assert_eq!(get_action(&["delete".to_string()]), "delete");
        assert_eq!(get_action(&["delete".to_string(), "create".to_string()]), "replace");
        assert_eq!(get_action(&["unknown".to_string()]), "unknown");
    }

    #[test]
    fn test_format_value() {
        assert_eq!(format_value(&json!("test")), "\"test\"");
        assert_eq!(format_value(&json!(42)), "42");
        assert_eq!(format_value(&json!(true)), "true");
        assert_eq!(format_value(&json!(null)), "null");
    }

    #[test]
    fn test_format_plan_collapsed() {
        let plan = create_test_plan();
        let output = format_plan(&plan, true);
        
        assert!(output.contains("▶ + aws_instance.web will be create"));
        assert!(output.contains("▶ ~ aws_s3_bucket.data will be update"));
        assert!(output.contains("Plan: 1 to add, 1 to change, 0 to destroy"));
        assert!(!output.contains("ami:"));
    }

    #[test]
    fn test_format_plan_expanded() {
        let plan = create_test_plan();
        let output = format_plan(&plan, false);
        
        assert!(output.contains("▼ + aws_instance.web will be create"));
        assert!(output.contains("ami: \"ami-12345678\""));
        assert!(output.contains("encryption: null => \"AES256\""));
        assert!(output.contains("versioning: false => true"));
    }

    #[test]
    fn test_empty_plan() {
        let plan = TerraformPlan {
            resource_changes: vec![],
        };
        let output = format_plan(&plan, false);
        
        assert!(output.contains("No changes. Your infrastructure matches the configuration"));
    }

    #[test]
    fn test_format_summary() {
        let mut counts = HashMap::new();
        counts.insert("create", 2);
        counts.insert("update", 1);
        counts.insert("delete", 1);
        
        let summary = format_summary(&counts);
        assert!(summary.contains("Plan: 2 to add, 1 to change, 1 to destroy"));
    }

    #[test]
    fn test_format_resource_change_create() {
        let change = ResourceChange {
            address: "aws_instance.test".to_string(),
            change: Change {
                actions: vec!["create".to_string()],
                before: None,
                after: Some(json!({"ami": "ami-123"})),
            },
        };
        
        let output = format_resource_change(&change, false);
        assert!(output.contains("▼ + aws_instance.test will be create"));
        assert!(output.contains("ami: \"ami-123\""));
    }

    #[test]
    fn test_format_resource_change_update() {
        let change = ResourceChange {
            address: "aws_instance.test".to_string(),
            change: Change {
                actions: vec!["update".to_string()],
                before: Some(json!({"size": "small"})),
                after: Some(json!({"size": "large"})),
            },
        };
        
        let output = format_resource_change(&change, false);
        assert!(output.contains("▼ ~ aws_instance.test will be update"));
        assert!(output.contains("size: \"small\" => \"large\""));
    }

    #[test]
    fn test_format_resource_change_delete() {
        let change = ResourceChange {
            address: "aws_instance.test".to_string(),
            change: Change {
                actions: vec!["delete".to_string()],
                before: Some(json!({"ami": "ami-123"})),
                after: None,
            },
        };
        
        let output = format_resource_change(&change, false);
        assert!(output.contains("▼ - aws_instance.test will be delete"));
    }

    #[test]
    fn test_format_resource_change_replace() {
        let change = ResourceChange {
            address: "aws_instance.test".to_string(),
            change: Change {
                actions: vec!["delete".to_string(), "create".to_string()],
                before: Some(json!({"ami": "ami-old"})),
                after: Some(json!({"ami": "ami-new"})),
            },
        };
        
        let output = format_resource_change(&change, false);
        assert!(output.contains("▼ -/+ aws_instance.test will be replace"));
        assert!(output.contains("ami: \"ami-old\" => \"ami-new\""));
    }
}