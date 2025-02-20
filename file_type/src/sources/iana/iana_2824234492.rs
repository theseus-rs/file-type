use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2824234492: FileType = FileType {
    file_format: &FileFormat {
        id: 2_824_234_492,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
