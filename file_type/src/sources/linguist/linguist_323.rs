use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_323: FileType = FileType {
    file_format: &FileFormat {
        id: 323,
        source_type: SourceType::Linguist,
        name: "RenderScript",
        extensions: &["rs", "rsh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
