use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2159550589: FileType = FileType {
    file_format: &FileFormat {
        id: 2_159_550_589,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.tableStyles+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
