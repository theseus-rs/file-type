use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1755453350: FileType = FileType {
    file_format: &FileFormat {
        id: 1_755_453_350,
        source_type: SourceType::Iana,
        name: "L24",
        extensions: &[],
        media_types: &["audio/L24"],
        signatures: &[],
        related_formats: &[],
    },
};
