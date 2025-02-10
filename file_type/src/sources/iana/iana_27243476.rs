use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_27243476: FileType = FileType {
    file_format: &FileFormat {
        id: 27_243_476,
        source_type: SourceType::Iana,
        name: "ohttp-req",
        extensions: &[],
        media_types: &["message/ohttp-req"],
        signatures: &[],
        related_formats: &[],
    },
};
