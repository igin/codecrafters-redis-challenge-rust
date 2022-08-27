use super::command_types::RESPValue;

pub fn serialize_resp(value: &RESPValue) -> String {
    match value {
        RESPValue::String(x) => format!("+{x}"),
        RESPValue::Error(x) => format!("-{}", x.message),
        RESPValue::Array(items) => {
            let values: Vec<String> = items.iter().map(serialize_resp).collect();
            values.join("\r\n")
        }
        RESPValue::NullString() => "$-1".to_string()
    }
}
