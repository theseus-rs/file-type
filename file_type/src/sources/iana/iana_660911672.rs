use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_660911672: FileType = FileType {
    file_format: &FileFormat {
        id: 660_911_672,
        source_type: SourceType::Iana,
        name: "csvm+json",
        extensions: &[],
        media_types: &["application/csvm+json"],
        signatures: &[],
        related_formats: &[],
    },
};
