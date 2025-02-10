use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3302833616: FileType = FileType {
    file_format: &FileFormat {
        id: 3_302_833_616,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
