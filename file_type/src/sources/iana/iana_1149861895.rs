use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1149861895: FileFormat = FileFormat {
    id: 1_149_861_895,
    source_type: SourceType::Iana,
    name: "tamp-sequence-adjust",
    extensions: &[],
    media_types: &["application/tamp-sequence-adjust"],
    signatures: &[],
    related_formats: &[],
};
