use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2928483447: FileType = FileType {
    file_format: &FileFormat {
        id: 2_928_483_447,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.wordprocessingml.settings+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
