use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1589663858: FileType = FileType {
    file_format: &FileFormat {
        id: 1_589_663_858,
        source_type: SourceType::Iana,
        name: "vnd.wordlift",
        extensions: &[],
        media_types: &["application/vnd.wordlift"],
        signatures: &[],
        related_formats: &[],
    },
};
