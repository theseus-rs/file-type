use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_391473714: FileType = FileType {
    file_format: &FileFormat {
        id: 391_473_714,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
