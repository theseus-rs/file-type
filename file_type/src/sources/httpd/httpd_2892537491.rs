use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2892537491: FileFormat = FileFormat {
    id: 2_892_537_491,
    source_type: SourceType::Httpd,
    name: "smaf",
    extensions: &["mmf"],
    media_types: &["application/vnd.smaf"],
    signatures: &[],
    related_formats: &[],
};
