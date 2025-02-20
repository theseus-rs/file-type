use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3396801964: FileType = FileType {
    file_format: &FileFormat {
        id: 3_396_801_964,
        source_type: SourceType::Iana,
        name: "atsc-rsat+xml",
        extensions: &[],
        media_types: &["application/atsc-rsat+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
