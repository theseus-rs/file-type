use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4036075207: FileFormat = FileFormat {
    id: 4_036_075_207,
    source_type: SourceType::Httpd,
    name: "vsf",
    extensions: &["vsf"],
    media_types: &["application/vnd.vsf"],
    internal_signatures: &[],
    related_formats: &[],
};
