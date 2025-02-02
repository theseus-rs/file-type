use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_605004890228926996: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "semf",
    extensions: &["semf"],
    media_types: &["application/vnd.semf"],
    internal_signatures: &[],
    related_formats: &[],
};
