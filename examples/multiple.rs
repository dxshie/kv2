use kv2::parse_kv2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DmElement {
    id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DmeModel {
    id: String,
    visible: bool,
}

fn main() {
    let input = r#"
"DmElement"
{
"id" "elementid" "df939bf4-8dd6-435c-9eef-a6e25434ecca"
"name" "string" "root"
}

"DmeModel"
{
"id" "elementid" "90e0ae34-0671-478d-95f5-12fa5c905c7a"
"visible" "bool" "1"
}
        "#;

    match parse_kv2(input) {
        Ok(data) => {
            let element = DmElement::deserialize(data.1[0].clone());
            let model = DmeModel::deserialize(data.1[1].clone());
            println!("{:#?}", element);
            println!("{:#?}", model);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
