use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_620086286: FileType = FileType {
    file_format: &FileFormat {
        id: 620_086_286,
        source_type: SourceType::Iana,
        name: "directory - DEPRECATED by RFC6350",
        extensions: &[],
        media_types: &["text/directory"],
        signatures: &[],
        related_formats: &[],
    },
};
