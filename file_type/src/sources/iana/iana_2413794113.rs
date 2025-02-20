use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2413794113: FileType = FileType {
    file_format: &FileFormat {
        id: 2_413_794_113,
        source_type: SourceType::Iana,
        name: "EVRCWB1",
        extensions: &[],
        media_types: &["audio/EVRCWB1"],
        signatures: &[],
        related_formats: &[],
    },
};
