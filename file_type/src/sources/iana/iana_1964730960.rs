use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1964730960: FileType = FileType {
    file_format: &FileFormat {
        id: 1_964_730_960,
        source_type: SourceType::Iana,
        name: "ATRAC-X",
        extensions: &[],
        media_types: &["audio/ATRAC-X"],
        signatures: &[],
        related_formats: &[],
    },
};
