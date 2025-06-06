use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_245146791: FileType = FileType {
    file_format: &FileFormat {
        id: 245_146_791,
        source_type: SourceType::Iana,
        name: "xliff+xml",
        extensions: &[],
        media_types: &["application/xliff+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
