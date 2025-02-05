use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1415644375: FileFormat = FileFormat {
    id: 1_415_644_375,
    source_type: SourceType::Httpd,
    name: "stardivision writer",
    extensions: &["sdw", "vor"],
    media_types: &["application/vnd.stardivision.writer"],
    signatures: &[],
    related_formats: &[],
};
