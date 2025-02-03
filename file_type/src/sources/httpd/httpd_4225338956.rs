use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4225338956: FileFormat = FileFormat {
    id: 4_225_338_956,
    source_type: SourceType::Httpd,
    name: "dxf",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
