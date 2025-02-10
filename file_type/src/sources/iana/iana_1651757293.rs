use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1651757293: FileType = FileType {
    file_format: &FileFormat {
        id: 1_651_757_293,
        source_type: SourceType::Iana,
        name: "vnd.ms-word.document.macroEnabled.12",
        extensions: &[],
        media_types: &["application/vnd.ms-word.document.macroEnabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
