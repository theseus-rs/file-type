use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2063648569: FileType = FileType {
    file_format: &FileFormat {
        id: 2_063_648_569,
        source_type: SourceType::Iana,
        name: "vnd.groove-tool-template",
        extensions: &[],
        media_types: &["application/vnd.groove-tool-template"],
        signatures: &[],
        related_formats: &[],
    },
};
