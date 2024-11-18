use kv2::parse_kv2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DmElement {
    id: String,
    name: String,
}

fn main() {
    let input = r#"
"DmElement"
{
"id" "elementid" "df939bf4-8dd6-435c-9eef-a6e25434ecca"
"name" "string" "root"
}
"#;

    match parse_kv2(input) {
        Ok(data) => {
            let element = DmElement::deserialize(data.1[0].clone());
            println!("{:#?}", element);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
