use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_15268548954167723934: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cml",
    extensions: &["cml"],
    media_types: &["chemical/x-cml"],
    internal_signatures: &[],
    related_formats: &[],
};
