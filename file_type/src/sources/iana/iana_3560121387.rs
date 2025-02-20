use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3560121387: FileType = FileType {
    file_format: &FileFormat {
        id: 3_560_121_387,
        source_type: SourceType::Iana,
        name: "L16",
        extensions: &[],
        media_types: &["audio/L16"],
        signatures: &[],
        related_formats: &[],
    },
};
