use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4147141993: FileFormat = FileFormat {
    id: 4_147_141_993,
    source_type: SourceType::Httpd,
    name: "sgi movie",
    extensions: &["movie"],
    media_types: &["video/x-sgi-movie"],
    signatures: &[],
    related_formats: &[],
};
