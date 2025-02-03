use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3068637523: FileFormat = FileFormat {
    id: 3_068_637_523,
    source_type: SourceType::Httpd,
    name: "gxf",
    extensions: &["gxf"],
    media_types: &["application/gxf"],
    internal_signatures: &[],
    related_formats: &[],
};
