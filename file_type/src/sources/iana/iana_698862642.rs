use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_698862642: FileType = FileType {
    file_format: &FileFormat {
        id: 698_862_642,
        source_type: SourceType::Iana,
        name: "vnd.d3m-problem",
        extensions: &[],
        media_types: &["application/vnd.d3m-problem"],
        signatures: &[],
        related_formats: &[],
    },
};
