use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1414686593: FileType = FileType {
    file_format: &FileFormat {
        id: 1_414_686_593,
        source_type: SourceType::Iana,
        name: "vnd.comicbook+zip",
        extensions: &[],
        media_types: &["application/vnd.comicbook+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
