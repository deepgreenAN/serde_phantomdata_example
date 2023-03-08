use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Video<T> {
    #[serde(bound(serialize = "T: Default + Into<String> + Clone"))]
    #[serde(bound(deserialize = "T: TryFrom<String>"))]
    file_type: SerdePhantomData<T>,
    file_name: String,
}

impl<T> Video<T> {
    fn new(file_name: String) -> Video<T> {
        Video {
            file_name,
            file_type: SerdePhantomData(PhantomData),
        }
    }
}

// -------------------------------------------------------------------------------------------------
// SerdePhantom

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
#[serde(bound(serialize = "T: Default + Into<String> + Clone"))]
#[serde(bound(deserialize = "T: TryFrom<String>"))]
struct SerdePhantomData<T>(PhantomData<T>);

impl<T> TryFrom<String> for SerdePhantomData<T>
where
    T: TryFrom<String>,
{
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let _unit_type: T = value.try_into().map_err(|_| "Parse Error".to_string())?;
        Ok(SerdePhantomData(PhantomData))
    }
}

impl<T> From<SerdePhantomData<T>> for String
where
    T: Default + Into<String>,
{
    fn from(_value: SerdePhantomData<T>) -> Self {
        T::default().into()
    }
}
// -------------------------------------------------------------------------------------------------
// Unit Types

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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
