use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3852266454: FileType = FileType {
    file_format: &FileFormat {
        id: 3_852_266_454,
        source_type: SourceType::Iana,
        name: "vnd.wordperfect",
        extensions: &[],
        media_types: &["application/vnd.wordperfect"],
        signatures: &[],
        related_formats: &[],
    },
};
