use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3688277181: FileType = FileType {
    file_format: &FileFormat {
        id: 3_688_277_181,
        source_type: SourceType::Iana,
        name: "flac",
        extensions: &[],
        media_types: &["audio/flac"],
        signatures: &[],
        related_formats: &[],
    },
};
