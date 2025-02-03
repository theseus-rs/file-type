use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2866207433: FileFormat = FileFormat {
    id: 2_866_207_433,
    source_type: SourceType::Httpd,
    name: "mediastation cdkey",
    extensions: &["cdkey"],
    media_types: &["application/vnd.mediastation.cdkey"],
    internal_signatures: &[],
    related_formats: &[],
};
