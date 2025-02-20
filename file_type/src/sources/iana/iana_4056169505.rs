use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4056169505: FileType = FileType {
    file_format: &FileFormat {
        id: 4_056_169_505,
        source_type: SourceType::Iana,
        name: "hjif",
        extensions: &[],
        media_types: &["haptics/hjif"],
        signatures: &[],
        related_formats: &[],
    },
};
