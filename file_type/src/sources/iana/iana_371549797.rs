use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_371549797: FileType = FileType {
    file_format: &FileFormat {
        id: 371_549_797,
        source_type: SourceType::Iana,
        name: "vnd.dece.video",
        extensions: &[],
        media_types: &["video/vnd.dece.video"],
        signatures: &[],
        related_formats: &[],
    },
};
