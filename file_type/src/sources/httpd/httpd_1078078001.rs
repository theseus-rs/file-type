use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1078078001: FileFormat = FileFormat {
    id: 1_078_078_001,
    source_type: SourceType::Httpd,
    name: "groove help",
    extensions: &["ghf"],
    media_types: &["application/vnd.groove-help"],
    signatures: &[],
    related_formats: &[],
};
