#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Video<T> {
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

// -------------------------------------------------------------------------------------------------
// Unit Types

#[derive(Debug, PartialEq)]
struct Mp4;

#[derive(Debug, PartialEq)]
struct Avi;

fn main() {
    let video = Video::<Mp4>::new("my video 1".to_string());

    assert_eq!(
        serde_json::to_string(&video).unwrap(),
        r#"{"file_type":null,"file_name":"my video 1"}"#
    );

    let video_json = r#"{"file_type":null,"file_name":"my video 2"}"#;

    assert_eq!(
        serde_json::from_str::<Video<Avi>>(video_json).unwrap(),
        Video::<Avi>::new("my video 2".to_string())
    )
}
