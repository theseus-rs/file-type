use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_15793840: FileType = FileType {
    file_format: &FileFormat {
        id: 15_793_840,
        source_type: SourceType::Iana,
        name: "vnd.nervana",
        extensions: &[],
        media_types: &["application/vnd.nervana"],
        signatures: &[],
        related_formats: &[],
    },
};
