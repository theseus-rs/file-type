use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_43: FileType = FileType {
    file_format: &FileFormat {
        id: 43,
        source_type: SourceType::Pronom,
        name: "Unicode Text File",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
