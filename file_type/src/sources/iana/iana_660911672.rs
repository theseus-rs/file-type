use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_660911672: FileFormat = FileFormat {
    id: 660_911_672,
    source_type: SourceType::Iana,
    name: "csvm+json",
    extensions: &[],
    media_types: &["application/csvm+json"],
    internal_signatures: &[],
    related_formats: &[],
};
