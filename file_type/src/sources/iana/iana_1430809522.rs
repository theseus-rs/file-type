use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1430809522: FileType = FileType {
    file_format: &FileFormat {
        id: 1_430_809_522,
        source_type: SourceType::Iana,
        name: "global-delivery-status",
        extensions: &[],
        media_types: &["message/global-delivery-status"],
        signatures: &[],
        related_formats: &[],
    },
};
