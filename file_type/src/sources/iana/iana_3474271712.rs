use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3474271712: FileType = FileType {
    file_format: &FileFormat {
        id: 3_474_271_712,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
