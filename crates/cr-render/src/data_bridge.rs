use cr_core::resume::Resume;
use serde_json::Value;
use std::path::Path;

/// Write resume data as JSON that the Typst template will consume.
/// Converts snake_case keys to kebab-case to avoid Typst underscore issues.
/// Converts photo path to just the filename (since file is copied to template dir).
pub fn write_data_json(resume: &Resume, out_dir: &Path) -> anyhow::Result<std::path::PathBuf> {
    let path = out_dir.join("resume-data.json");
    let value = serde_json::to_value(resume)?;
    let mut converted = convert_keys(value);

    // Convert photo path to just the filename
    if let Some(personal) = converted.get_mut("personal") {
        if let Some(photo) = personal.get("photo") {
            if let Some(photo_str) = photo.as_str() {
                let filename = std::path::Path::new(photo_str)
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or(photo_str);
                personal["photo"] = Value::String(filename.to_string());
            }
        }
    }

    let json = serde_json::to_string_pretty(&converted)?;
    std::fs::write(&path, json)?;
    Ok(path)
}

/// Recursively convert all snake_case keys to kebab-case
fn convert_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let new_map: serde_json::Map<String, Value> = map
                .into_iter()
                .map(|(k, v)| (k.replace('_', "-"), convert_keys(v)))
                .collect();
            Value::Object(new_map)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(convert_keys).collect()),
        other => other,
    }
}
