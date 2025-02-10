use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4154079289: FileType = FileType {
    file_format: &FileFormat {
        id: 4_154_079_289,
        source_type: SourceType::Iana,
        name: "clue_info+xml",
        extensions: &[],
        media_types: &["application/clue_info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
