use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1623729236: FileType = FileType {
    file_format: &FileFormat {
        id: 1_623_729_236,
        source_type: SourceType::Iana,
        name: "vnd.lotus-1-2-3",
        extensions: &[],
        media_types: &["application/vnd.lotus-1-2-3"],
        signatures: &[],
        related_formats: &[],
    },
};
