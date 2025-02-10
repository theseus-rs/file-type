use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2496902356: FileType = FileType {
    file_format: &FileFormat {
        id: 2_496_902_356,
        source_type: SourceType::Iana,
        name: "soap+xml",
        extensions: &[],
        media_types: &["application/soap+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
