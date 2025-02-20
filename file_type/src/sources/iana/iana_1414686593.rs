use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
