use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1160699760: FileType = FileType {
    file_format: &FileFormat {
        id: 1_160_699_760,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.wordprocessingml.comments+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
