use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2823327473: FileType = FileType {
    file_format: &FileFormat {
        id: 2_823_327_473,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
