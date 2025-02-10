use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_854397495: FileType = FileType {
    file_format: &FileFormat {
        id: 854_397_495,
        source_type: SourceType::Iana,
        name: "parameters",
        extensions: &[],
        media_types: &["text/parameters"],
        signatures: &[],
        related_formats: &[],
    },
};
