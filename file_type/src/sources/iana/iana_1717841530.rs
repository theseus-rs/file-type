use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1717841530: FileType = FileType {
    file_format: &FileFormat {
        id: 1_717_841_530,
        source_type: SourceType::Iana,
        name: "tamp-error",
        extensions: &[],
        media_types: &["application/tamp-error"],
        signatures: &[],
        related_formats: &[],
    },
};
