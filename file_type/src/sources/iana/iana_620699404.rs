use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_620699404: FileType = FileType {
    file_format: &FileFormat {
        id: 620_699_404,
        source_type: SourceType::Iana,
        name: "dialog-info+xml",
        extensions: &[],
        media_types: &["application/dialog-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
