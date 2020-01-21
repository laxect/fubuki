use crate::fetch_agent::FetchRequest;

const version_check_uri: &'static str = "https://tenzan.gyara.moe/key/blog_version";

pub fn new() -> FetchRequest {
    FetchRequest::Uncacheable(version_check_uri.to_owned())
}
