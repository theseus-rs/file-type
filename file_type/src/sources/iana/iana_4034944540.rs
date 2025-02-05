use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4034944540: FileFormat = FileFormat {
    id: 4_034_944_540,
    source_type: SourceType::Iana,
    name: "FFV1",
    extensions: &[],
    media_types: &["video/FFV1"],
    signatures: &[],
    related_formats: &[],
};
