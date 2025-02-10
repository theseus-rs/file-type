use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1628407127: FileType = FileType {
    file_format: &FileFormat {
        id: 1_628_407_127,
        source_type: SourceType::Iana,
        name: "mathml-presentation+xml",
        extensions: &[],
        media_types: &["application/mathml-presentation+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
