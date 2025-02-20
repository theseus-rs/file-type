use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2570913583: FileType = FileType {
    file_format: &FileFormat {
        id: 2_570_913_583,
        source_type: SourceType::Iana,
        name: "delivery-status",
        extensions: &[],
        media_types: &["message/delivery-status"],
        signatures: &[],
        related_formats: &[],
    },
};
