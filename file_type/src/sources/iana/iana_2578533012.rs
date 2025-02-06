use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2578533012: FileFormat = FileFormat {
    id: 2_578_533_012,
    source_type: SourceType::Iana,
    name: "vnd.kde.kontour",
    extensions: &[],
    media_types: &["application/vnd.kde.kontour"],
    signatures: &[],
    related_formats: &[],
};
