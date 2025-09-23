// use serde_json::Value;

// pub fn decode(data: Json<Value>) -> Option<Value> {
//     // Decode percent-encoded JSON string, similar to the Python logic
//     let mut s = String::from_utf8_lossy(data).to_string();
//     s = s.replace("result=", "")
//         .replace("%5B", "[")
//         .replace("%5D", "]")
//         .replace("%7B", "{")
//         .replace("%7D", "}")
//         .replace("%22", "\"")
//         .replace("%3A", ":")
//         .replace("%2C", ",")
//         .replace("+", " ")
//         .replace("%5C", "\\")
//         .replace("%3E", ">")
//         .replace("%3C", "<")
//         .replace("%23", "#")
//         .replace("%7C", "|")
//         .replace("%3D", "=")
//         .replace("%21", "!");

//     let mut result: Value = match serde_json::from_str(&s) {
//         Ok(val) => val,
//         Err(_) => return None,
//     };

//     // Split log["first"] and log["last"] by newlines, if present
//     if let Some(log) = result.get_mut("log") {
//         if let Some(first) = log.get_mut("first") {
//             if let Some(arr) = first.as_array_mut() {
//                 for log_entry in arr.iter_mut() {
//                     if let Some(s) = log_entry.as_str() {
//                         if s.contains('\n') {
//                             let split: Vec<Value> = s.split('\n').map(|x| Value::String(x.to_string())).collect();
//                             *log_entry = Value::Array(split);
//                         }
//                     }
//                 }
//             }
//         }
//         if let Some(last) = log.get_mut("last") {
//             if let Some(arr) = last.as_array_mut() {
//                 for log_entry in arr.iter_mut() {
//                     if let Some(s) = log_entry.as_str() {
//                         let split: Vec<Value> = s.split('\n').map(|x| Value::String(x.to_string())).collect();
//                         *log_entry = Value::Array(split);
//                     }
//                 }
//             }
//         }
//     }

//     Some(result)
// }
