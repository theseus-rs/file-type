use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2645641524: FileFormat = FileFormat {
    id: 2_645_641_524,
    source_type: SourceType::Httpd,
    name: "mxf",
    extensions: &["mxf"],
    media_types: &["application/mxf"],
    signatures: &[],
    related_formats: &[],
};
