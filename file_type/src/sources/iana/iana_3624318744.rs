use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3624318744: FileType = FileType {
    file_format: &FileFormat {
        id: 3_624_318_744,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
