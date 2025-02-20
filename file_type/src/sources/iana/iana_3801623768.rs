use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3801623768: FileType = FileType {
    file_format: &FileFormat {
        id: 3_801_623_768,
        source_type: SourceType::Iana,
        name: "vnd.d3m-dataset",
        extensions: &[],
        media_types: &["application/vnd.d3m-dataset"],
        signatures: &[],
        related_formats: &[],
    },
};
