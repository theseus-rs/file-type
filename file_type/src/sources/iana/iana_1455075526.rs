use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1455075526: FileType = FileType {
    file_format: &FileFormat {
        id: 1_455_075_526,
        source_type: SourceType::Iana,
        name: "obj",
        extensions: &[],
        media_types: &["model/obj"],
        signatures: &[],
        related_formats: &[],
    },
};
