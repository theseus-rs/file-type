use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1006546886522380100: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "apple installer xml",
    extensions: &["mpkg"],
    media_types: &["application/vnd.apple.installer+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
