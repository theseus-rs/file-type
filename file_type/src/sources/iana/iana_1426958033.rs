use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1426958033: FileType = FileType {
    file_format: &FileFormat {
        id: 1_426_958_033,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
