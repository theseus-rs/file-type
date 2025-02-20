use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
