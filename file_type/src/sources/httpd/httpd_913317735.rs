use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_913317735: FileFormat = FileFormat {
    id: 913_317_735,
    source_type: SourceType::Httpd,
    name: "doom",
    extensions: &["wad"],
    media_types: &["application/x-doom"],
    internal_signatures: &[],
    related_formats: &[],
};
