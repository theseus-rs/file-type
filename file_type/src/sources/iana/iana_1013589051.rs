use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1013589051: FileType = FileType {
    file_format: &FileFormat {
        id: 1_013_589_051,
        source_type: SourceType::Iana,
        name: "wordperfect5.1",
        extensions: &[],
        media_types: &["application/wordperfect5.1"],
        signatures: &[],
        related_formats: &[],
    },
};
