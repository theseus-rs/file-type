use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2846680230: FileType = FileType {
    file_format: &FileFormat {
        id: 2_846_680_230,
        source_type: SourceType::Iana,
        name: "mets+xml",
        extensions: &[],
        media_types: &["application/mets+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
