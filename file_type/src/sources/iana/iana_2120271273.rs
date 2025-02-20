use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2120271273: FileType = FileType {
    file_format: &FileFormat {
        id: 2_120_271_273,
        source_type: SourceType::Iana,
        name: "MF4",
        extensions: &[],
        media_types: &["application/MF4"],
        signatures: &[],
        related_formats: &[],
    },
};
