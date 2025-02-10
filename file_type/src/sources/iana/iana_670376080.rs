use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_670376080: FileType = FileType {
    file_format: &FileFormat {
        id: 670_376_080,
        source_type: SourceType::Iana,
        name: "sp-midi",
        extensions: &[],
        media_types: &["audio/sp-midi"],
        signatures: &[],
        related_formats: &[],
    },
};
