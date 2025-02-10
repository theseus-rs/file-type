use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4034476122: FileType = FileType {
    file_format: &FileFormat {
        id: 4_034_476_122,
        source_type: SourceType::Iana,
        name: "vnd.apple.pages",
        extensions: &[],
        media_types: &["application/vnd.apple.pages"],
        signatures: &[],
        related_formats: &[],
    },
};
