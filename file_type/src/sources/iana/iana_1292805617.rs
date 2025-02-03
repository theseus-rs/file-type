use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1292805617: FileFormat = FileFormat {
    id: 1_292_805_617,
    source_type: SourceType::Iana,
    name: "alto-directory+json",
    extensions: &[],
    media_types: &["application/alto-directory+json"],
    internal_signatures: &[],
    related_formats: &[],
};
