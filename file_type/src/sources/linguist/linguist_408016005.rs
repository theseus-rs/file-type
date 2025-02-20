use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_408016005: FileType = FileType {
    file_format: &FileFormat {
        id: 408_016_005,
        source_type: SourceType::Linguist,
        name: "VBScript",
        extensions: &["vbs"],
        media_types: &["text/vbscript"],
        signatures: &[],
        related_formats: &[],
    },
};
