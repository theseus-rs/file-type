use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4229936947: FileFormat = FileFormat {
    id: 4_229_936_947,
    source_type: SourceType::Httpd,
    name: "shana informed interchange",
    extensions: &["iif"],
    media_types: &["application/vnd.shana.informed.interchange"],
    internal_signatures: &[],
    related_formats: &[],
};
