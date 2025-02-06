use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2447158225: FileFormat = FileFormat {
    id: 2_447_158_225,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.drawing+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.drawing+xml"],
    signatures: &[],
    related_formats: &[],
};
