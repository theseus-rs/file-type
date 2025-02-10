use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3967535666: FileType = FileType {
    file_format: &FileFormat {
        id: 3_967_535_666,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
