use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_971985811: FileFormat = FileFormat {
    id: 971_985_811,
    source_type: SourceType::Iana,
    name: "clue+xml",
    extensions: &[],
    media_types: &["application/clue+xml"],
    signatures: &[],
    related_formats: &[],
};
