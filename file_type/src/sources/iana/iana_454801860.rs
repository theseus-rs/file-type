use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_454801860: FileFormat = FileFormat {
    id: 454_801_860,
    source_type: SourceType::Iana,
    name: "vnd.fst",
    extensions: &[],
    media_types: &["image/vnd.fst"],
    signatures: &[],
    related_formats: &[],
};
