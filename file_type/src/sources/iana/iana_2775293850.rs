use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2775293850: FileType = FileType {
    file_format: &FileFormat {
        id: 2_775_293_850,
        source_type: SourceType::Iana,
        name: "troff",
        extensions: &[],
        media_types: &["text/troff"],
        signatures: &[],
        related_formats: &[],
    },
};
