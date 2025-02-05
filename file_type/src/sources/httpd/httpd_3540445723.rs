use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3540445723: FileFormat = FileFormat {
    id: 3_540_445_723,
    source_type: SourceType::Httpd,
    name: "font snf",
    extensions: &["snf"],
    media_types: &["application/x-font-snf"],
    signatures: &[],
    related_formats: &[],
};
