use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2763672296: FileType = FileType {
    file_format: &FileFormat {
        id: 2_763_672_296,
        source_type: SourceType::Iana,
        name: "call-completion",
        extensions: &[],
        media_types: &["application/call-completion"],
        signatures: &[],
        related_formats: &[],
    },
};
