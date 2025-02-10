use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3309875328: FileType = FileType {
    file_format: &FileFormat {
        id: 3_309_875_328,
        source_type: SourceType::Iana,
        name: "G726-40",
        extensions: &[],
        media_types: &["audio/G726-40"],
        signatures: &[],
        related_formats: &[],
    },
};
