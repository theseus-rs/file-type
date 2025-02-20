use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_49797597: FileType = FileType {
    file_format: &FileFormat {
        id: 49_797_597,
        source_type: SourceType::Iana,
        name: "mathml-content+xml",
        extensions: &[],
        media_types: &["application/mathml-content+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
