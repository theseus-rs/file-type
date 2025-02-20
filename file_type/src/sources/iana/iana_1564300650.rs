use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1564300650: FileType = FileType {
    file_format: &FileFormat {
        id: 1_564_300_650,
        source_type: SourceType::Iana,
        name: "EVRCNW1",
        extensions: &[],
        media_types: &["audio/EVRCNW1"],
        signatures: &[],
        related_formats: &[],
    },
};
