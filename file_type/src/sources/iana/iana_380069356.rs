use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_380069356: FileType = FileType {
    file_format: &FileFormat {
        id: 380_069_356,
        source_type: SourceType::Iana,
        name: "aces",
        extensions: &[],
        media_types: &["image/aces"],
        signatures: &[],
        related_formats: &[],
    },
};
