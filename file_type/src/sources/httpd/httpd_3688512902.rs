use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3688512902: FileFormat = FileFormat {
    id: 3_688_512_902,
    source_type: SourceType::Httpd,
    name: "sh",
    extensions: &["sh"],
    media_types: &["application/x-sh"],
    signatures: &[],
    related_formats: &[],
};
