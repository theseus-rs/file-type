use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1785152264: FileType = FileType {
    file_format: &FileFormat {
        id: 1_785_152_264,
        source_type: SourceType::Iana,
        name: "vnd.ctc-posml",
        extensions: &[],
        media_types: &["application/vnd.ctc-posml"],
        signatures: &[],
        related_formats: &[],
    },
};
