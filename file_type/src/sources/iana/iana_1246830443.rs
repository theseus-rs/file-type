use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1246830443: FileType = FileType {
    file_format: &FileFormat {
        id: 1_246_830_443,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
