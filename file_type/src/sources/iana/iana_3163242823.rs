use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3163242823: FileType = FileType {
    file_format: &FileFormat {
        id: 3_163_242_823,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
