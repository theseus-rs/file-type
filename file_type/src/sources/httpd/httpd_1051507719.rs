use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1051507719: FileFormat = FileFormat {
    id: 1_051_507_719,
    source_type: SourceType::Httpd,
    name: "flv",
    extensions: &["flv"],
    media_types: &["video/x-flv"],
    internal_signatures: &[],
    related_formats: &[],
};
