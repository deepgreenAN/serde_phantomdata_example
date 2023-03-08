use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Video<T> {
    #[serde(
        serialize_with = "serialize_phantom",
        deserialize_with = "deserialize_phantom"
    )]
    #[serde(bound(serialize = "T: Into<String> + Default"))]
    #[serde(bound(deserialize = "T: TryFrom<String>"))]
    file_type: PhantomData<T>,
    file_name: String,
}

impl<T> Video<T> {
    fn new(file_name: String) -> Video<T> {
        Video {
            file_name,
            file_type: PhantomData,
        }
    }
}

fn serialize_phantom<S, T>(_: &PhantomData<T>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Into<String> + Default,
{
    let type_str: String = T::default().into();
    s.serialize_str(&type_str)
}

fn deserialize_phantom<'de, D, T>(d: D) -> Result<PhantomData<T>, D::Error>
where
    D: Deserializer<'de>,
    T: TryFrom<String>,
{
    let unit_type_str: String = Deserialize::deserialize(d)?;
    let _unit_type: T = unit_type_str
        .try_into()
        .map_err(|_| serde::de::Error::custom("Parse Error".to_string()))?;

    Ok(PhantomData)
}

// -------------------------------------------------------------------------------------------------
// Unit Types

#[derive(Debug, Default, PartialEq)]
struct Mp4;

impl TryFrom<String> for Mp4 {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.as_str() == stringify!(Mp4) {
            Ok(Mp4)
        } else {
            Err("Parse Error".to_string())
        }
    }
}

impl From<Mp4> for String {
    fn from(_: Mp4) -> Self {
        stringify!(Mp4).to_string()
    }
}

#[derive(Debug, Default, PartialEq)]
struct Avi;

impl TryFrom<String> for Avi {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.as_str() == stringify!(Avi) {
            Ok(Avi)
        } else {
            Err("Parse Error".to_string())
        }
    }
}

impl From<Avi> for String {
    fn from(_: Avi) -> Self {
        stringify!(Avi).to_string()
    }
}

fn main() {
    let video = Video::<Mp4>::new("my video 1".to_string());

    assert_eq!(
        serde_json::to_string(&video).unwrap(),
        r#"{"file_type":"Mp4","file_name":"my video 1"}"#
    );

    let video_json = r#"{"file_type":"Avi","file_name":"my video 2"}"#;

    assert_eq!(
        serde_json::from_str::<Video<Avi>>(video_json).unwrap(),
        Video::<Avi>::new("my video 2".to_string())
    )
}
