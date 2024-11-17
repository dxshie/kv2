use serde::de::{self, DeserializeSeed, Deserializer, IntoDeserializer, MapAccess, Visitor};
use serde::{forward_to_deserialize_any, Deserialize};
use std::collections::HashMap;
use std::fmt;

use crate::{parse_kv2, KV2Object, KV2Value};

impl<'de> Deserializer<'de> for KV2Object {
    type Error = de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(KV2ObjectMapAccess {
            iter: self.fields.into_iter(),
            value: None,
        })
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq
        tuple tuple_struct map struct enum identifier ignored_any
    }
}

struct KV2ObjectMapAccess {
    iter: std::collections::hash_map::IntoIter<String, KV2Value>,
    value: Option<KV2Value>,
}

impl<'de> MapAccess<'de> for KV2ObjectMapAccess {
    type Error = de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.iter.next() {
            self.value = Some(value);
            Ok(Some(seed.deserialize(key.into_deserializer())?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(value) = self.value.take() {
            seed.deserialize(value)
        } else {
            Err(de::Error::custom("Value is missing for KV2Object map"))
        }
    }
}

struct KV2ValueSeqAccess {
    iter: std::vec::IntoIter<KV2Value>,
}

impl<'de> de::SeqAccess<'de> for KV2ValueSeqAccess {
    type Error = de::value::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(value) = self.iter.next() {
            seed.deserialize(value).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct KV2VectorSeqAccess {
    iter: std::vec::IntoIter<f64>,
}

impl<'de> de::SeqAccess<'de> for KV2VectorSeqAccess {
    type Error = de::value::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(value) = self.iter.next() {
            seed.deserialize(value.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl<'de> Deserializer<'de> for KV2Value {
    type Error = de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            KV2Value::Bool(b) => visitor.visit_bool(b),
            KV2Value::Int(i) => visitor.visit_i64(i),
            KV2Value::Double(d) => visitor.visit_f64(d),
            KV2Value::String(s) => visitor.visit_string(s),
            KV2Value::Array(arr) => visitor.visit_seq(KV2ValueSeqAccess {
                iter: arr.into_iter(),
            }),
            KV2Value::Object(obj) => visitor.visit_map(KV2ObjectMapAccess {
                iter: obj.fields.into_iter(),
                value: None,
            }),
            KV2Value::Vector(v) => visitor.visit_seq(KV2VectorSeqAccess {
                iter: v.into_iter(),
            }),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq
        tuple tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> Deserialize<'de> for KV2Value {
    fn deserialize<D>(deserializer: D) -> Result<KV2Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KV2ValueVisitor;

        impl<'de> Visitor<'de> for KV2ValueVisitor {
            type Value = KV2Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a KV2 value")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
                Ok(KV2Value::Bool(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(KV2Value::Int(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(KV2Value::Int(v as i64))
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
                Ok(KV2Value::Double(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(KV2Value::String(v.to_string()))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
                Ok(KV2Value::String(v))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element()? {
                    values.push(value);
                }
                Ok(KV2Value::Array(values))
            }

            fn visit_map<M>(self, mut map: M) -> Result<KV2Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut fields = HashMap::new();
                while let Some((key, value)) = map.next_entry()? {
                    fields.insert(key, value);
                }
                Ok(KV2Value::Object(KV2Object {
                    class_name: String::new(), // Class name might be empty here
                    fields,
                }))
            }
        }

        deserializer.deserialize_any(KV2ValueVisitor)
    }
}

pub fn serde_kv2<'de, T>(input: &'de str) -> Result<T, Box<dyn std::error::Error + 'de>>
where
    T: Deserialize<'de>,
{
    // Parse the KV2 data
    let (_, parsed_kv2) = parse_kv2(input)?;

    // Deserialize directly into the target struct
    let result: T = T::deserialize(parsed_kv2)?;

    Ok(result)
}
