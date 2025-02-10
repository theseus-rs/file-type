use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1115896451: FileType = FileType {
    file_format: &FileFormat {
        id: 1_115_896_451,
        source_type: SourceType::Iana,
        name: "vnd.keyman.kmp+zip",
        extensions: &[],
        media_types: &["application/vnd.keyman.kmp+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
