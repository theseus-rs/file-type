use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1916595166: FileType = FileType {
    file_format: &FileFormat {
        id: 1_916_595_166,
        source_type: SourceType::Iana,
        name: "riscos",
        extensions: &[],
        media_types: &["application/riscos"],
        signatures: &[],
        related_formats: &[],
    },
};
