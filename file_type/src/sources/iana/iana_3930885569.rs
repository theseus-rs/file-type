use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3930885569: FileFormat = FileFormat {
    id: 3_930_885_569,
    source_type: SourceType::Iana,
    name: "mixed",
    extensions: &[],
    media_types: &["multipart/mixed"],
    signatures: &[],
    related_formats: &[],
};
