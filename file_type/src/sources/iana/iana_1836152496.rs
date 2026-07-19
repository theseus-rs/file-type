use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1836152496: FileType = FileType {
    file_format: &FileFormat {
        id: 1_836_152_496,
        source_type: SourceType::Iana,
        name: "vnd.aia",
        extensions: &[],
        media_types: &["application/vnd.aia"],
        signatures: &[],
        related_formats: &[],
    },
};
