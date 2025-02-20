use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2008460097: FileType = FileType {
    file_format: &FileFormat {
        id: 2_008_460_097,
        source_type: SourceType::Iana,
        name: "batch-SMTP",
        extensions: &[],
        media_types: &["application/batch-SMTP"],
        signatures: &[],
        related_formats: &[],
    },
};
