use percent_encoding::percent_decode_str;

pub mod auth;
pub mod common;
pub mod exchange;
pub mod feedback;
pub mod payments;
pub mod transfer;
pub mod user;

fn replace_placeholder(path_segments: Vec<&str>, value: String, placeholder: &str) -> String {
    path_segments
        .iter()
        .map(|&segment| {
            if percent_decode_str(segment).decode_utf8_lossy() == placeholder {
                value.clone()
            } else {
                segment.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}
