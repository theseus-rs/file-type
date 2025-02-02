use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8668725384356574970: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mxf",
    extensions: &["mxf"],
    media_types: &["application/mxf"],
    internal_signatures: &[],
    related_formats: &[],
};
