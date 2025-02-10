use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2909660000: FileType = FileType {
    file_format: &FileFormat {
        id: 2_909_660_000,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
