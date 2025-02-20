use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_40: FileType = FileType {
    file_format: &FileFormat {
        id: 40,
        source_type: SourceType::Pronom,
        name: "Tab-separated Values",
        extensions: &["tsv", "tab"],
        media_types: &["text/tab-separated-values"],
        signatures: &[],
        related_formats: &[],
    },
};
