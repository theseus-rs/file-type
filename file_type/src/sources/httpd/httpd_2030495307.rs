use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2030495307: FileFormat = FileFormat {
    id: 2_030_495_307,
    source_type: SourceType::Httpd,
    name: "apple installer xml",
    extensions: &["mpkg"],
    media_types: &["application/vnd.apple.installer+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
