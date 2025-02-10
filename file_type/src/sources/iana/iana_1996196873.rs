use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1996196873: FileType = FileType {
    file_format: &FileFormat {
        id: 1_996_196_873,
        source_type: SourceType::Iana,
        name: "bhttp",
        extensions: &[],
        media_types: &["message/bhttp"],
        signatures: &[],
        related_formats: &[],
    },
};
