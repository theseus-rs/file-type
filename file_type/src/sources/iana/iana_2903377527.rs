use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2903377527: FileType = FileType {
    file_format: &FileFormat {
        id: 2_903_377_527,
        source_type: SourceType::Iana,
        name: "geo+json-seq",
        extensions: &[],
        media_types: &["application/geo+json-seq"],
        signatures: &[],
        related_formats: &[],
    },
};
