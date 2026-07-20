use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2918820000: FileType = FileType {
    file_format: &FileFormat {
        id: 2_918_820_000,
        source_type: SourceType::Iana,
        name: "vnd.apple.steering-list",
        extensions: &[],
        media_types: &["application/vnd.apple.steering-list"],
        signatures: &[],
        related_formats: &[],
    },
};
