use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_150129542: FileType = FileType {
    file_format: &FileFormat {
        id: 150_129_542,
        source_type: SourceType::Iana,
        name: "vnd.nokia.conml+xml",
        extensions: &[],
        media_types: &["application/vnd.nokia.conml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
