use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1804860659: FileFormat = FileFormat {
    id: 1_804_860_659,
    source_type: SourceType::Iana,
    name: "vnd.ms-cab-compressed",
    extensions: &[],
    media_types: &["application/vnd.ms-cab-compressed"],
    signatures: &[],
    related_formats: &[],
};
