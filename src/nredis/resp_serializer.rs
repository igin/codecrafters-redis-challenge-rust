use super::command_types::RESPValue;

pub fn serialize_resp(value: &RESPValue) -> String {
    match value {
        RESPValue::String(x) => x.to_string(),
        RESPValue::Error(x) => x.message.to_string(),
        RESPValue::Array(items) => {
            let values: Vec<String> = items
                .iter()
                .map(|x| serialize_resp(&x).to_string())
                .collect();
            let joined = values.join("\r\n");
            return joined;
        }
    }
}
