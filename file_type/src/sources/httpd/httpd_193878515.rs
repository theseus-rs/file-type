use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_193878515: FileFormat = FileFormat {
    id: 193_878_515,
    source_type: SourceType::Httpd,
    name: "sdp",
    extensions: &["sdp"],
    media_types: &["application/sdp"],
    signatures: &[],
    related_formats: &[],
};
