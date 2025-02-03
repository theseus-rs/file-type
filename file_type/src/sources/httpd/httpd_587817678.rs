use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_587817678: FileFormat = FileFormat {
    id: 587_817_678,
    source_type: SourceType::Httpd,
    name: "nfo",
    extensions: &["nfo"],
    media_types: &["text/x-nfo"],
    internal_signatures: &[],
    related_formats: &[],
};
