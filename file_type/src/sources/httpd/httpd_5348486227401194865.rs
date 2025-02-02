use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5348486227401194865: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "smaf",
    extensions: &["mmf"],
    media_types: &["application/vnd.smaf"],
    internal_signatures: &[],
    related_formats: &[],
};
