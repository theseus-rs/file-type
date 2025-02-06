use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1550746427: FileFormat = FileFormat {
    id: 1_550_746_427,
    source_type: SourceType::Iana,
    name: "pkixcmp",
    extensions: &[],
    media_types: &["application/pkixcmp"],
    signatures: &[],
    related_formats: &[],
};
