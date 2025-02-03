use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3834962646: FileFormat = FileFormat {
    id: 3_834_962_646,
    source_type: SourceType::Httpd,
    name: "ssml xml",
    extensions: &["ssml"],
    media_types: &["application/ssml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
