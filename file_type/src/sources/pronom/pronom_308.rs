use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_308: FileType = FileType {
    file_format: &FileFormat {
        id: 308,
        source_type: SourceType::Pronom,
        name: "Adobe ACD",
        extensions: &["acd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
