use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2860056698: FileType = FileType {
    file_format: &FileFormat {
        id: 2_860_056_698,
        source_type: SourceType::Iana,
        name: "usac",
        extensions: &[],
        media_types: &["audio/usac"],
        signatures: &[],
        related_formats: &[],
    },
};
