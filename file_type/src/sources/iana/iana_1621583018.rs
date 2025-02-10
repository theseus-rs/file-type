use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1621583018: FileType = FileType {
    file_format: &FileFormat {
        id: 1_621_583_018,
        source_type: SourceType::Iana,
        name: "t140c",
        extensions: &[],
        media_types: &["audio/t140c"],
        signatures: &[],
        related_formats: &[],
    },
};
