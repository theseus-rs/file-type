use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_18215989259559031662: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ssml xml",
    extensions: &["ssml"],
    media_types: &["application/ssml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
