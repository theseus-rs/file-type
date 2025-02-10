use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3248699242: FileType = FileType {
    file_format: &FileFormat {
        id: 3_248_699_242,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.connections+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
