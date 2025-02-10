use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3859418788: FileType = FileType {
    file_format: &FileFormat {
        id: 3_859_418_788,
        source_type: SourceType::Iana,
        name: "n-triples",
        extensions: &[],
        media_types: &["application/n-triples"],
        signatures: &[],
        related_formats: &[],
    },
};
