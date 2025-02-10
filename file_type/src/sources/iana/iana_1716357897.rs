use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1716357897: FileType = FileType {
    file_format: &FileFormat {
        id: 1_716_357_897,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.presentationml.slideMaster+xml",
        extensions: &[],
        media_types: &[
            "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
