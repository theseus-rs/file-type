use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_209: FileType = FileType {
    file_format: &FileFormat {
        id: 209,
        source_type: SourceType::Linguist,
        name: "Logos",
        extensions: &["x", "xi", "xm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
