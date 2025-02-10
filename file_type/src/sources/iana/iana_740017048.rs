use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_740017048: FileType = FileType {
    file_format: &FileFormat {
        id: 740_017_048,
        source_type: SourceType::Iana,
        name: "node",
        extensions: &[],
        media_types: &["application/node"],
        signatures: &[],
        related_formats: &[],
    },
};
