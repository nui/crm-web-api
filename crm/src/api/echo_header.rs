use serde::ser::SerializeTuple;
use serde::{Serialize, Serializer};
use warp::http::HeaderMap;
use warp::reply;
use warp::reply::WithHeader;

/// A test route that echo safe http headers
pub fn echo_header(header_map: HeaderMap) -> WithHeader<String> {
    // header that are safe to echo back
    const SAFE_HEADERS: &[&str] = &[
        "accept",
        "accept-encoding",
        "accept-language",
        "connection",
        "host",
        "x-forwarded-for",
        "x-real-ip",
    ];
    let mut safe_headers = Vec::with_capacity(header_map.len());
    let body = measure_time!("serialize safe headers", {
        for (k, v) in header_map.iter() {
            if let (key, Ok(value)) = (k.as_str(), v.to_str()) {
                if SAFE_HEADERS.contains(&key) {
                    safe_headers.push(SafeHeaderValue { name: key, value })
                }
            }
        }
        serde_json::to_string_pretty(&RequestInfo {
            safe_headers: &safe_headers,
        })
        .unwrap()
    });
    reply::with_header(body, "Content-Type", "application/json")
}

struct SafeHeaderValue<'a> {
    name: &'a str,
    value: &'a str,
}

#[derive(Serialize)]
struct RequestInfo<'a> {
    #[serde(serialize_with = "serialize_safe_header_value_slice")]
    safe_headers: &'a [SafeHeaderValue<'a>],
}

fn serialize_safe_header_value_slice<'a, S>(
    slice: &'a [SafeHeaderValue<'a>],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_tuple(slice.len())?;
    let mut buf = String::new();
    for &SafeHeaderValue { name, value } in slice {
        buf.clear();
        buf.push_str(name);
        buf.push_str(": ");
        buf.push_str(value);
        seq.serialize_element(&buf)?;
    }
    seq.end()
}
