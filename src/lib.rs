#[cfg(feature = "serde")]
pub mod kv2_serde;

mod test;

use std::collections::HashMap;

use log::info;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace1,
    combinator::{map, opt},
    multi::{many0, separated_list0},
    sequence::delimited,
    IResult,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "serde"))]
#[derive(Debug)]
pub enum KV2Value {
    Bool(bool),
    Int(i64),
    Double(f64),
    Vector(Vec<f64>),
    String(String),
    Array(Vec<KV2Value>),
    Object(KV2Object),
}

#[cfg(feature = "serde")]
#[derive(Debug, Serialize)]
pub enum KV2Value {
    Bool(bool),
    Int(i64),
    Double(f64),
    Vector(Vec<f64>),
    String(String),
    Array(Vec<KV2Value>),
    Object(KV2Object),
}

#[cfg(feature = "serde")]
#[derive(Debug, Serialize, Deserialize)]
pub struct KV2Object {
    pub class_name: String,
    pub fields: HashMap<String, KV2Value>,
}

#[cfg(not(feature = "serde"))]
#[derive(Debug)]
pub struct KV2Object {
    pub class_name: String,
    pub fields: HashMap<String, KV2Value>,
}

pub fn parse_kv2(input: &str) -> IResult<&str, KV2Object> {
    info!("Parsing KV2 root...");

    let (input, _) = skip_comments_and_whitespace(input)?;

    // Parse optional XML-style comment at the top
    let (input, _) = opt(parse_comment)(input)?;

    // Parse the root class name
    let (input, class_name) = ws(parse_quoted_string)(input)?;

    // Parse the main object body
    let (input, fields) = parse_object_body(input)?;

    Ok((input, KV2Object { class_name, fields }))
}

fn parse_object_body(input: &str) -> IResult<&str, HashMap<String, KV2Value>> {
    let (input, _) = ws(tag("{"))(input)?;
    let (input, kvs) = many0(ws(parse_key_value_or_entry))(input)?;
    let (input, _) = ws(tag("}"))(input)?;
    Ok((input, kvs.into_iter().collect()))
}

fn parse_key_value_or_entry(input: &str) -> IResult<&str, (String, KV2Value)> {
    // Try to parse a key-value pair first, then an array, then an object
    alt((
        parse_key_value,
        parse_array,
        parse_object_with_classname_as_value,
    ))(input)
}

fn parse_key_value(input: &str) -> IResult<&str, (String, KV2Value)> {
    info!("Parsing key-value pair...");

    let (input, key) = ws(parse_quoted_string)(input)?;
    let (input, data_type) = ws(parse_quoted_string)(input)?;
    let (input, value_str) = ws(parse_quoted_string)(input)?;

    let value = match data_type.as_str() {
        "bool" => KV2Value::Bool(value_str == "1" || value_str.to_lowercase() == "true"),
        "int" | "int32" | "int64" => KV2Value::Int(value_str.parse::<i64>().unwrap_or(0)),
        "float" => KV2Value::Double(value_str.parse::<f64>().unwrap_or(0.0)),
        "string" => KV2Value::String(value_str),
        "elementid" => KV2Value::String(value_str), // Treat element IDs as strings
        "vector3" => {
            // Parse the vector string into a Vec<f64>
            match parse_vector(value_str.as_str()) {
                Ok(vector) => KV2Value::Vector(vector),
                Err(_) => KV2Value::String(value_str), // Fallback to string if parsing fails
            }
        }
        // Handle other data types as needed
        _ => KV2Value::String(value_str), // Default to string
    };

    Ok((input, (key, value)))
}
fn parse_vector(input: &str) -> Result<Vec<f64>, std::num::ParseFloatError> {
    input.split_whitespace().map(|s| s.parse::<f64>()).collect()
}

fn parse_array(input: &str) -> IResult<&str, (String, KV2Value)> {
    info!("Parsing array...");
    let (input, key) = ws(parse_quoted_string)(input)?;
    let (input, data_type) = ws(parse_quoted_string)(input)?;

    if data_type != "element_array" {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let (input, _) = ws(tag("["))(input)?;
    // Use separated_list0 to handle commas between elements
    let (input, elements) = separated_list0(ws(tag(",")), ws(parse_element))(input)?;
    let (input, _) = ws(tag("]"))(input)?;

    Ok((input, (key, KV2Value::Array(elements))))
}

fn parse_element(input: &str) -> IResult<&str, KV2Value> {
    info!("Parsing element...");
    // Parse the class name
    let (input, class_name) = ws(parse_quoted_string)(input)?;
    // Parse the object body
    let (input, fields) = parse_object_body(input)?;
    Ok((input, KV2Value::Object(KV2Object { class_name, fields })))
}

fn parse_object_with_classname_as_value(input: &str) -> IResult<&str, (String, KV2Value)> {
    info!("Parsing object with classname...");
    // Parse the key
    let (input, key) = ws(parse_quoted_string)(input)?;
    // Parse the data type (should be the class name)
    let (input, data_type) = ws(parse_quoted_string)(input)?;
    // Parse the object body
    let (input, fields) = parse_object_body(input)?;
    Ok((
        input,
        (
            key,
            KV2Value::Object(KV2Object {
                class_name: data_type,
                fields,
            }),
        ),
    ))
}

fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    info!("Parsing quoted string...");
    let result = delimited(tag("\""), take_until("\""), tag("\""))(input);
    result.map(|(remaining, s)| (remaining, s.to_string()))
}

fn parse_comment(input: &str) -> IResult<&str, ()> {
    // Parse XML-style comments (<!-- ... -->)
    let mut xml_style = map(
        delimited(tag("<!--"), take_until("-->"), tag("-->")),
        |_| (), // Ignore content
    );

    xml_style(input)
}

fn skip_comments_and_whitespace(input: &str) -> IResult<&str, ()> {
    map(
        many0(alt((map(multispace1, |_| ()), parse_comment))),
        |_| (),
    )(input)
}

fn ws<'a, F: 'a, O>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, O>
where
    F: Fn(&'a str) -> IResult<&'a str, O>,
{
    move |input: &str| {
        let (input, _) = skip_comments_and_whitespace(input)?;
        let (input, res) = inner(input)?;
        let (input, _) = skip_comments_and_whitespace(input)?;
        Ok((input, res))
    }
}
