use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2572911062: FileFormat = FileFormat {
    id: 2_572_911_062,
    source_type: SourceType::Iana,
    name: "vnd.kde.kivio",
    extensions: &[],
    media_types: &["application/vnd.kde.kivio"],
    signatures: &[],
    related_formats: &[],
};
