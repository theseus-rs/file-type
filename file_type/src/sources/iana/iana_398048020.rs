use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_398048020: FileType = FileType {
    file_format: &FileFormat {
        id: 398_048_020,
        source_type: SourceType::Iana,
        name: "vnd.pocketlearn",
        extensions: &[],
        media_types: &["application/vnd.pocketlearn"],
        signatures: &[],
        related_formats: &[],
    },
};
