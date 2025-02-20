use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3834896132: FileType = FileType {
    file_format: &FileFormat {
        id: 3_834_896_132,
        source_type: SourceType::Iana,
        name: "json-seq",
        extensions: &[],
        media_types: &["application/json-seq"],
        signatures: &[],
        related_formats: &[],
    },
};
