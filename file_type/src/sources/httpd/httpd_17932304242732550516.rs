use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17932304242732550516: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "vsf",
    extensions: &["vsf"],
    media_types: &["application/vnd.vsf"],
    internal_signatures: &[],
    related_formats: &[],
};
