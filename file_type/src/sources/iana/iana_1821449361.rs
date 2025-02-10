use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1821449361: FileType = FileType {
    file_format: &FileFormat {
        id: 1_821_449_361,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
