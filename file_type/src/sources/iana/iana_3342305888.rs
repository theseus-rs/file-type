use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3342305888: FileType = FileType {
    file_format: &FileFormat {
        id: 3_342_305_888,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
