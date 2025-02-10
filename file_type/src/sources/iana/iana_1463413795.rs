use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1463413795: FileType = FileType {
    file_format: &FileFormat {
        id: 1_463_413_795,
        source_type: SourceType::Iana,
        name: "vnd.futoin+json",
        extensions: &[],
        media_types: &["application/vnd.futoin+json"],
        signatures: &[],
        related_formats: &[],
    },
};
