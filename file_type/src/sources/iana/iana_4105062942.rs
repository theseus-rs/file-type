use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4105062942: FileType = FileType {
    file_format: &FileFormat {
        id: 4_105_062_942,
        source_type: SourceType::Iana,
        name: "matroska",
        extensions: &[],
        media_types: &["audio/matroska"],
        signatures: &[],
        related_formats: &[],
    },
};
