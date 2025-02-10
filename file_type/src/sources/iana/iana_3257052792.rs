use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3257052792: FileType = FileType {
    file_format: &FileFormat {
        id: 3_257_052_792,
        source_type: SourceType::Iana,
        name: "DAT12",
        extensions: &[],
        media_types: &["audio/DAT12"],
        signatures: &[],
        related_formats: &[],
    },
};
